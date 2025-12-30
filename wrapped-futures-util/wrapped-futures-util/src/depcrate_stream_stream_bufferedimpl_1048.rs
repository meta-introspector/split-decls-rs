// Generated macro for impl_1048 (impl)
macro_rules! Depcrate_stream_stream_bufferedimpl_1048 {
() => {
// Module: crate::stream::stream::buffered
// Provides: {"impl_1048"}
// Dependencies: {}
impl < St > Buffered < St > where St : Stream , St :: Item : Future , { pub (super) fn new (stream : St , n : Option < usize >) -> Self { Self { stream : super :: Fuse :: new (stream) , in_progress_queue : FuturesOrdered :: new () , max : n . and_then (NonZeroUsize :: new) , } } delegate_access_inner ! (stream , St , (.)) ; }
};
}
