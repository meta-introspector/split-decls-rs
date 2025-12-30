// Generated macro for impl_313 (impl)
macro_rules! Depcrate_concurrent_stream_limitimpl_313 {
() => {
// Module: crate::concurrent_stream::limit
// Provides: {"impl_313"}
// Dependencies: {}
impl < CS : ConcurrentStream > ConcurrentStream for Limit < CS > { type Item = CS :: Item ; type Future = CS :: Future ; async fn drive < C > (self , consumer : C) -> C :: Output where C : Consumer < Self :: Item , Self :: Future > , { self . inner . drive (LimitConsumer { inner : consumer }) . await } fn concurrency_limit (& self) -> Option < NonZeroUsize > { self . limit } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
