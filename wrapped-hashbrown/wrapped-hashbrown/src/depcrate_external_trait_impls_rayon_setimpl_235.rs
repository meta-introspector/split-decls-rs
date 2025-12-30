// Generated macro for impl_235 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_setimpl_235 {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"impl_235"}
// Dependencies: {}
impl < T : Send , A : Allocator + Send + Sync > ParallelIterator for ParDrain < '_ , T , A > { type Item = T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| (k , _) | k) . drive_unindexed (consumer) } }
};
}
