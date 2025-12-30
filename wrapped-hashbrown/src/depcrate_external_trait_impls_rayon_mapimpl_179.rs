// Generated macro for impl_179 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_179 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_179"}
// Dependencies: {}
impl < 'a , K : Sync , V : Send > ParallelIterator for ParValuesMut < 'a , K , V > { type Item = & 'a mut V ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { & mut x . as_mut () . 1 }) . drive_unindexed (consumer) } }
};
}
