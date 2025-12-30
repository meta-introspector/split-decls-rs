// Generated macro for impl_1030 (impl)
macro_rules! Depcrate_stream_stream_buffer_unorderedimpl_1030 {
() => {
// Module: crate::stream::stream::buffer_unordered
// Provides: {"impl_1030"}
// Dependencies: {}
impl < St > BufferUnordered < St > where St : Stream , St :: Item : Future , { pub (super) fn new (stream : St , n : Option < usize >) -> Self { Self { stream : super :: Fuse :: new (stream) , in_progress_queue : FuturesUnordered :: new () , max : n . and_then (NonZeroUsize :: new) , } } delegate_access_inner ! (stream , St , (.)) ; }
};
}
