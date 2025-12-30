// Generated macro for impl_159 (impl)
macro_rules! Depcrate_async_readyimpl_159 {
() => {
// Module: crate::async_ready
// Provides: {"impl_159"}
// Dependencies: {}
impl IAsyncAction { # [doc = " Creates an `IAsyncAction` that is immediately ready with a value."] pub fn ready (result : Result < () >) -> Self { ReadyAction (ReadyState :: new (result)) . into () } }
};
}
