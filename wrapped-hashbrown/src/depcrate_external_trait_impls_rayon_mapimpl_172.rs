// Generated macro for impl_172 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_172 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_172"}
// Dependencies: {}
impl < 'a , K : Sync , V : Sync > ParallelIterator for ParValues < 'a , K , V > { type Item = & 'a V ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { & x . as_ref () . 1 }) . drive_unindexed (consumer) } }
};
}
