// Generated macro for impl_1384 (impl)
macro_rules! Depcrate_stream_try_stream_try_ready_chunksimpl_1384 {
() => {
// Module: crate::stream::try_stream::try_ready_chunks
// Provides: {"impl_1384"}
// Dependencies: {}
impl < St : TryStream > TryReadyChunks < St > { pub (super) fn new (stream : St , capacity : usize) -> Self { assert ! (capacity > 0) ; Self { stream : IntoStream :: new (stream) . fuse () , cap : capacity } } delegate_access_inner ! (stream , St , (. .)) ; }
};
}
