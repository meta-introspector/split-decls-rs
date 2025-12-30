// Generated macro for impl_233 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_setimpl_233 {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"impl_233"}
// Dependencies: {}
impl < T : Send , A : Allocator + Send > ParallelIterator for IntoParIter < T , A > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| (k , _) | k) . drive_unindexed (consumer) } }
};
}
