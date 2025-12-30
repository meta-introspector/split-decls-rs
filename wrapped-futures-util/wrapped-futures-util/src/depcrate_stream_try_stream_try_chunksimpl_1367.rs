// Generated macro for impl_1367 (impl)
macro_rules! Depcrate_stream_try_stream_try_chunksimpl_1367 {
() => {
// Module: crate::stream::try_stream::try_chunks
// Provides: {"impl_1367"}
// Dependencies: {}
impl < St : TryStream + FusedStream > FusedStream for TryChunks < St > { fn is_terminated (& self) -> bool { self . stream . is_terminated () && self . items . is_empty () } }
};
}
