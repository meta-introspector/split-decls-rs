// Generated macro for impl_1364 (impl)
macro_rules! Depcrate_stream_try_stream_try_chunksimpl_1364 {
() => {
// Module: crate::stream::try_stream::try_chunks
// Provides: {"impl_1364"}
// Dependencies: {}
impl < St : TryStream > TryChunks < St > { pub (super) fn new (stream : St , capacity : usize) -> Self { assert ! (capacity > 0) ; Self { stream : IntoStream :: new (stream) . fuse () , items : Vec :: with_capacity (capacity) , cap : capacity , } } fn take (self : Pin < & mut Self >) -> Vec < St :: Ok > { let cap = self . cap ; mem :: replace (self . project () . items , Vec :: with_capacity (cap)) } delegate_access_inner ! (stream , St , (. .)) ; }
};
}
