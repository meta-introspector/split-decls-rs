// Generated macro for IntoAsyncStream (trait)
macro_rules! Depcrate_processIntoAsyncStream {
() => {
// Module: crate::process
// Provides: {"IntoAsyncStream"}
// Dependencies: {}
# [cfg (feature = "async")] # [doc = " IntoAsyncStream interface turns a [Process::Stream] into an async version."] # [doc = " To be used with `async`/`await`syntax"] pub trait IntoAsyncStream { # [doc = " AsyncStream type."] # [doc = " Like [Process::Stream] but it represents an async IO stream."] type AsyncStream ; # [doc = " Turns an object into a async stream."] fn into_async_stream (self) -> Result < Self :: AsyncStream > ; }
};
}
