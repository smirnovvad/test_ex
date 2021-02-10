#![feature(async_closure)]
use futures::{Future, stream::{StreamExt}};


const THRESHOLD: usize = 2;

async fn test<T, F, Fut>(t: &Vec<T>, f: F) -> Vec<T>
where
    T: Send + Clone + Copy,
    F: FnOnce(T) -> Fut + Copy,
    Fut: Future<Output = T> + Send,
{
    futures::stream::iter(t)
        .map(|&i| f(i))
        .buffer_unordered(THRESHOLD)
        .collect()
        .await
}

#[tokio::main]
async fn main() {
    let v: Vec<i32> = (0..1000000).collect();
    test(&v, async move |x| x.swap_bytes()).await;
}
