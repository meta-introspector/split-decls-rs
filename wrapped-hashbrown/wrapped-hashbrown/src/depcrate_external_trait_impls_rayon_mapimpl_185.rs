// Generated macro for impl_185 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_185 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_185"}
// Dependencies: {}
impl < K : Send , V : Send , A : Allocator + Sync > ParallelIterator for ParDrain < '_ , K , V , A > { type Item = (K , V) ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . drive_unindexed (consumer) } }
};
}
