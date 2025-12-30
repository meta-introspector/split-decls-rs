// Generated macro for impl_719 (impl)
macro_rules! Depcrate_stream_stream_forwardimpl_719 {
() => {
// Module: crate::stream::stream::forward
// Provides: {"impl_719"}
// Dependencies: {}
impl < St , Si , Item > Forward < St , Si , Item > { pub (crate) fn new (stream : St , sink : Si) -> Self { Self { sink : Some (sink) , stream : Fuse :: new (stream) , buffered_item : None } } }
};
}
