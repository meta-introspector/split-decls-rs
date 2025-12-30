// Generated macro for impl_168 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_168 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_168"}
// Dependencies: {}
impl < 'a , K : Sync , V : Sync > ParallelIterator for ParKeys < 'a , K , V > { type Item = & 'a K ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { & x . as_ref () . 0 }) . drive_unindexed (consumer) } }
};
}
