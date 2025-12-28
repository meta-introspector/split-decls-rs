macro_rules! AsyncFuture {
    () => {
        pub struct AsyncFuture < A : Async > { inner : A , status : IAsyncInfo , waker : Option < Arc < Mutex < Waker > > > , }
    };
}

AsyncFuture!()