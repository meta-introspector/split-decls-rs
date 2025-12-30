// Generated macro for impl_200 (impl)
macro_rules! Depcrate_futureimpl_200 {
() => {
// Module: crate::future
// Provides: {"impl_200"}
// Dependencies: {}
impl < T : RuntimeType > IntoFuture for IAsyncOperation < T > { type Output = Result < T > ; type IntoFuture = AsyncFuture < Self > ; fn into_future (self) -> Self :: IntoFuture { AsyncFuture :: new (self) } }
};
}
