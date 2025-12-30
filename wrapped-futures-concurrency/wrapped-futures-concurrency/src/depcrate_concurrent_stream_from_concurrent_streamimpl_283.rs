// Generated macro for impl_283 (impl)
macro_rules! Depcrate_concurrent_stream_from_concurrent_streamimpl_283 {
() => {
// Module: crate::concurrent_stream::from_concurrent_stream
// Provides: {"impl_283"}
// Dependencies: {}
impl < 'a , Fut : Future > VecConsumer < 'a , Fut > { pub (crate) fn new (output : & 'a mut Vec < Fut :: Output >) -> Self { Self { group : FuturesUnordered :: new () , output , } } }
};
}
