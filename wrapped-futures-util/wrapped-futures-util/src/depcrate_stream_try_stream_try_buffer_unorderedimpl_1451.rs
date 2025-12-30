// Generated macro for impl_1451 (impl)
macro_rules! Depcrate_stream_try_stream_try_buffer_unorderedimpl_1451 {
() => {
// Module: crate::stream::try_stream::try_buffer_unordered
// Provides: {"impl_1451"}
// Dependencies: {}
impl < St > TryBufferUnordered < St > where St : TryStream , St :: Ok : TryFuture , { pub (super) fn new (stream : St , n : Option < usize >) -> Self { Self { stream : IntoStream :: new (stream) . fuse () , in_progress_queue : FuturesUnordered :: new () , max : n . and_then (NonZeroUsize :: new) , } } delegate_access_inner ! (stream , St , (. .)) ; }
};
}
