// Generated macro for AsyncFuture (struct)
macro_rules! Depcrate_futureAsyncFuture {
() => {
// Module: crate::future
// Provides: {"AsyncFuture"}
// Dependencies: {}
pub struct AsyncFuture < A : Async > { inner : A , status : IAsyncInfo , waker : Option < Arc < Mutex < Waker > > > , }
};
}
