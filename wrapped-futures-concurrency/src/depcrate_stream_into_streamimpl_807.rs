// Generated macro for impl_807 (impl)
macro_rules! Depcrate_stream_into_streamimpl_807 {
() => {
// Module: crate::stream::into_stream
// Provides: {"impl_807"}
// Dependencies: {}
impl < S : Stream > IntoStream for S { type Item = S :: Item ; type IntoStream = S ; # [inline] fn into_stream (self) -> S { self } }
};
}
