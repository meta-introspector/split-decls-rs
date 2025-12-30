// Generated macro for impl_176 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_mapimpl_176 {
() => {
// Module: crate::external_trait_impls::rayon::map
// Provides: {"impl_176"}
// Dependencies: {}
impl < 'a , K : Sync , V : Send > ParallelIterator for ParIterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { let r = x . as_mut () ; (& r . 0 , & mut r . 1) }) . drive_unindexed (consumer) } }
};
}
