// Generated macro for impl_286 (impl)
macro_rules! Depcrate_concurrent_stream_from_concurrent_streamimpl_286 {
() => {
// Module: crate::concurrent_stream::from_concurrent_stream
// Provides: {"impl_286"}
// Dependencies: {}
impl < 'a , Fut : Future , T , E > ResultVecConsumer < 'a , Fut , T , E > { pub (crate) fn new (output : & 'a mut Result < Vec < T > , E >) -> Self { Self { group : FuturesUnordered :: new () , output , } } }
};
}
