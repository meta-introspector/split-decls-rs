// Generated macro for impl_983 (impl)
macro_rules! Depcrate_stream_stream_chunksimpl_983 {
() => {
// Module: crate::stream::stream::chunks
// Provides: {"impl_983"}
// Dependencies: {}
impl < St : FusedStream > FusedStream for Chunks < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () && self . items . is_empty () } }
};
}
