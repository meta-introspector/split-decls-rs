// Generated macro for impl_160 (impl)
macro_rules! Depcrate_async_readyimpl_160 {
() => {
// Module: crate::async_ready
// Provides: {"impl_160"}
// Dependencies: {}
impl < T : RuntimeType > IAsyncOperation < T > { # [doc = " Creates an `IAsyncOperation<T>` that is immediately ready with a value."] pub fn ready (result : Result < T >) -> Self { ReadyOperation (ReadyState :: new (result)) . into () } }
};
}
