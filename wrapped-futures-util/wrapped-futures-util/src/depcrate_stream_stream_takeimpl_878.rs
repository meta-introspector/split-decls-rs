// Generated macro for impl_878 (impl)
macro_rules! Depcrate_stream_stream_takeimpl_878 {
() => {
// Module: crate::stream::stream::take
// Provides: {"impl_878"}
// Dependencies: {}
impl < St : Stream > Take < St > { pub (super) fn new (stream : St , n : usize) -> Self { Self { stream , remaining : n } } delegate_access_inner ! (stream , St , ()) ; }
};
}
