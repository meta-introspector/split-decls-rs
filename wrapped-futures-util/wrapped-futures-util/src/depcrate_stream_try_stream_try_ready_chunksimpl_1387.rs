// Generated macro for impl_1387 (impl)
macro_rules! Depcrate_stream_try_stream_try_ready_chunksimpl_1387 {
() => {
// Module: crate::stream::try_stream::try_ready_chunks
// Provides: {"impl_1387"}
// Dependencies: {}
impl < St : TryStream + FusedStream > FusedStream for TryReadyChunks < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
};
}
