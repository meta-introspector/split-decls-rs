// Generated macro for impl_161 (impl)
macro_rules! Depcrate_async_readyimpl_161 {
() => {
// Module: crate::async_ready
// Provides: {"impl_161"}
// Dependencies: {}
impl < P : RuntimeType > IAsyncActionWithProgress < P > { # [doc = " Creates an `IAsyncActionWithProgress<P>` that is immediately ready with a value."] pub fn ready (result : Result < () >) -> Self { ReadyActionWithProgress (ReadyState :: new (result)) . into () } }
};
}
