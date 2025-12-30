// Generated macro for impl_997 (impl)
macro_rules! Depcrate_stream_stream_ready_chunksimpl_997 {
() => {
// Module: crate::stream::stream::ready_chunks
// Provides: {"impl_997"}
// Dependencies: {}
impl < St : Stream > FusedStream for ReadyChunks < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
