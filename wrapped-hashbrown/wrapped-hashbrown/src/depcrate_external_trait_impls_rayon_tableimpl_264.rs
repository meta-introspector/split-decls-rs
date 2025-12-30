// Generated macro for impl_264 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_tableimpl_264 {
() => {
// Module: crate::external_trait_impls::rayon::table
// Provides: {"impl_264"}
// Dependencies: {}
impl < 'a , T : Sync > ParallelIterator for ParIter < 'a , T > { type Item = & 'a T ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { x . as_ref () }) . drive_unindexed (consumer) } }
};
}
