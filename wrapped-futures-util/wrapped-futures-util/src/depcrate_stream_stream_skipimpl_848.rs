// Generated macro for impl_848 (impl)
macro_rules! Depcrate_stream_stream_skipimpl_848 {
() => {
// Module: crate::stream::stream::skip
// Provides: {"impl_848"}
// Dependencies: {}
impl < St : Stream > Skip < St > { pub (super) fn new (stream : St , n : usize) -> Self { Self { stream , remaining : n } } delegate_access_inner ! (stream , St , ()) ; }
};
}
