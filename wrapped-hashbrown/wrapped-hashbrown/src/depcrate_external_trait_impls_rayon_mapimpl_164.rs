// Generated macro for impl_164 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_164 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_164"}
// Dependencies: {}
impl < 'a , K : Sync , V : Sync > ParallelIterator for ParIter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { let r = x . as_ref () ; (& r . 0 , & r . 1) }) . drive_unindexed (consumer) } }
};
}
