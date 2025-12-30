// Generated macro for impl_199 (impl)
macro_rules! Depcrate_futureimpl_199 {
() => {
// Module: crate::future
// Provides: {"impl_199"}
// Dependencies: {}
impl IntoFuture for IAsyncAction { type Output = Result < () > ; type IntoFuture = AsyncFuture < Self > ; fn into_future (self) -> Self :: IntoFuture { AsyncFuture :: new (self) } }
};
}
