// Generated macro for impl_274 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_tableimpl_274 {
() => {
// Module: crate::external_trait_impls::rayon::table
// Provides: {"impl_274"}
// Dependencies: {}
impl < T : Send , A : Allocator + Sync > ParallelIterator for ParDrain < '_ , T , A > { type Item = T ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . drive_unindexed (consumer) } }
};
}
