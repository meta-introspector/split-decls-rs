// Generated macro for impl_201 (impl)
macro_rules! Depcrate_futureimpl_201 {
() => {
// Module: crate::future
// Provides: {"impl_201"}
// Dependencies: {}
impl < P : RuntimeType > IntoFuture for IAsyncActionWithProgress < P > { type Output = Result < () > ; type IntoFuture = AsyncFuture < Self > ; fn into_future (self) -> Self :: IntoFuture { AsyncFuture :: new (self) } }
};
}
