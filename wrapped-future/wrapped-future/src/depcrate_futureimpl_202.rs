// Generated macro for impl_202 (impl)
macro_rules! Depcrate_futureimpl_202 {
() => {
// Module: crate::future
// Provides: {"impl_202"}
// Dependencies: {}
impl < T : RuntimeType , P : RuntimeType > IntoFuture for IAsyncOperationWithProgress < T , P > { type Output = Result < T > ; type IntoFuture = AsyncFuture < Self > ; fn into_future (self) -> Self :: IntoFuture { AsyncFuture :: new (self) } }
};
}
