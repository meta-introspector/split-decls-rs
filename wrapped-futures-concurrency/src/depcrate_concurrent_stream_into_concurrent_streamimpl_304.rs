// Generated macro for impl_304 (impl)
macro_rules! Depcrate_concurrent_stream_into_concurrent_streamimpl_304 {
() => {
// Module: crate::concurrent_stream::into_concurrent_stream
// Provides: {"impl_304"}
// Dependencies: {}
impl < S : ConcurrentStream > IntoConcurrentStream for S { type Item = S :: Item ; type IntoConcurrentStream = S ; fn into_co_stream (self) -> Self :: IntoConcurrentStream { self } }
};
}
