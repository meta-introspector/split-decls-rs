// Generated macro for impl_162 (impl)
macro_rules! Depcrate_async_readyimpl_162 {
() => {
// Module: crate::async_ready
// Provides: {"impl_162"}
// Dependencies: {}
impl < T : RuntimeType , P : RuntimeType > IAsyncOperationWithProgress < T , P > { # [doc = " Creates an `IAsyncOperationWithProgress<T, P>` that is immediately ready with a value."] pub fn ready (result : Result < T >) -> Self { ReadyOperationWithProgress (ReadyState :: new (result)) . into () } }
};
}
