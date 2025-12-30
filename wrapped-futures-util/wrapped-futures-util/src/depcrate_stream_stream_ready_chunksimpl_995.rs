// Generated macro for impl_995 (impl)
macro_rules! Depcrate_stream_stream_ready_chunksimpl_995 {
() => {
// Module: crate::stream::stream::ready_chunks
// Provides: {"impl_995"}
// Dependencies: {}
impl < St : Stream > ReadyChunks < St > { pub (super) fn new (stream : St , capacity : usize) -> Self { assert ! (capacity > 0) ; Self { stream : stream . fuse () , cap : capacity } } delegate_access_inner ! (stream , St , (.)) ; }
};
}
