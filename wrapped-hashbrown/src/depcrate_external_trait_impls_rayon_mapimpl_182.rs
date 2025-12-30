// Generated macro for impl_182 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_182 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_182"}
// Dependencies: {}
impl < K : Send , V : Send , A : Allocator + Send > ParallelIterator for IntoParIter < K , V , A > { type Item = (K , V) ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . drive_unindexed (consumer) } }
};
}
