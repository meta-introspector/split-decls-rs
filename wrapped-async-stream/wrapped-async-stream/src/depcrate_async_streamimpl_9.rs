// Generated macro for impl_9 (impl)
macro_rules! Depcrate_async_streamimpl_9 {
() => {
// Module: crate::async_stream
// Provides: {"impl_9"}
// Dependencies: {}
impl < T , U > AsyncStream < T , U > { # [doc (hidden)] pub fn new (rx : Receiver < T > , generator : U) -> AsyncStream < T , U > { AsyncStream { rx , done : false , generator , } } }
};
}
