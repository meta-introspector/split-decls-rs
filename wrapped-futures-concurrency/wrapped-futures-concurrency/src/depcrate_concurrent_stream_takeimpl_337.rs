// Generated macro for impl_337 (impl)
macro_rules! Depcrate_concurrent_stream_takeimpl_337 {
() => {
// Module: crate::concurrent_stream::take
// Provides: {"impl_337"}
// Dependencies: {}
impl < CS : ConcurrentStream > ConcurrentStream for Take < CS > { type Item = CS :: Item ; type Future = CS :: Future ; async fn drive < C > (self , consumer : C) -> C :: Output where C : Consumer < Self :: Item , Self :: Future > , { self . inner . drive (TakeConsumer { inner : consumer , count : 0 , limit : self . limit , }) . await } fn concurrency_limit (& self) -> Option < NonZeroUsize > { self . inner . concurrency_limit () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
