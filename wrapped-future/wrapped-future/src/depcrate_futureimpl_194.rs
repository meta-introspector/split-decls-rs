// Generated macro for impl_194 (impl)
macro_rules! Depcrate_futureimpl_194 {
() => {
// Module: crate::future
// Provides: {"impl_194"}
// Dependencies: {}
impl < A : Async > AsyncFuture < A > { fn new (inner : A) -> Self { Self { status : inner . cast () . unwrap () , inner , waker : None , } } }
};
}
