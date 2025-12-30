// Generated macro for impl_1466 (impl)
macro_rules! Depcrate_stream_try_stream_try_bufferedimpl_1466 {
() => {
// Module: crate::stream::try_stream::try_buffered
// Provides: {"impl_1466"}
// Dependencies: {}
impl < St > TryBuffered < St > where St : TryStream , St :: Ok : TryFuture , { pub (super) fn new (stream : St , n : Option < usize >) -> Self { Self { stream : IntoStream :: new (stream) . fuse () , in_progress_queue : FuturesOrdered :: new () , max : n . and_then (NonZeroUsize :: new) , } } delegate_access_inner ! (stream , St , (. .)) ; }
};
}
