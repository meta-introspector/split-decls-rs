// Generated macro for impl_1268 (impl)
macro_rules! Depcrate_stream_try_stream_try_forwardimpl_1268 {
() => {
// Module: crate::stream::try_stream::try_forward
// Provides: {"impl_1268"}
// Dependencies: {}
impl < St , Si , Item > TryForward < St , Si , Item > { pub (crate) fn new (stream : St , sink : Si) -> Self { Self { sink : Some (sink) , stream : Fuse :: new (IntoStream :: new (stream)) , buffered_item : None } } }
};
}
