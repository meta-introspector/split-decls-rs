// Generated macro for impl_245 (impl)
macro_rules! Depcrate_concurrent_stream_enumerateimpl_245 {
() => {
// Module: crate::concurrent_stream::enumerate
// Provides: {"impl_245"}
// Dependencies: {}
impl < CS : ConcurrentStream > ConcurrentStream for Enumerate < CS > { type Item = (usize , CS :: Item) ; type Future = EnumerateFuture < CS :: Future , CS :: Item > ; async fn drive < C > (self , consumer : C) -> C :: Output where C : Consumer < Self :: Item , Self :: Future > , { self . inner . drive (EnumerateConsumer { inner : consumer , count : 0 , }) . await } fn concurrency_limit (& self) -> Option < NonZeroUsize > { self . inner . concurrency_limit () } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
