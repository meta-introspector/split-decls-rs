// Generated macro for impl_17 (impl)
macro_rules! Depcrate_collections_vecimpl_17 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_17"}
// Dependencies: {}
impl < T > ConcurrentStream for IntoConcurrentStream < T > { type Item = T ; type Future = Ready < T > ; async fn drive < C > (self , consumer : C) -> C :: Output where C : concurrent_stream :: Consumer < Self :: Item , Self :: Future > , { self . 0 . drive (consumer) . await } fn concurrency_limit (& self) -> Option < core :: num :: NonZeroUsize > { self . 0 . concurrency_limit () } }
};
}
