// Generated macro for impl_849 (impl)
macro_rules! Depcrate_stream_stream_skipimpl_849 {
() => {
// Module: crate::stream::stream::skip
// Provides: {"impl_849"}
// Dependencies: {}
impl < St : FusedStream > FusedStream for Skip < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
