// Generated macro for impl_981 (impl)
macro_rules! Depcrate_stream_stream_chunksimpl_981 {
() => {
// Module: crate::stream::stream::chunks
// Provides: {"impl_981"}
// Dependencies: {}
impl < St : Stream > Chunks < St > { pub (super) fn new (stream : St , capacity : usize) -> Self { assert ! (capacity > 0) ; Self { stream : super :: Fuse :: new (stream) , items : Vec :: with_capacity (capacity) , cap : capacity , } } fn take (self : Pin < & mut Self >) -> Vec < St :: Item > { let cap = self . cap ; mem :: replace (self . project () . items , Vec :: with_capacity (cap)) } delegate_access_inner ! (stream , St , (.)) ; }
};
}
