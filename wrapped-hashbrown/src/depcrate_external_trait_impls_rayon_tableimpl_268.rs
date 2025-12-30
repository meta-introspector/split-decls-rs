// Generated macro for impl_268 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_tableimpl_268 {
() => {
// Module: crate::external_trait_impls::rayon::table
// Provides: {"impl_268"}
// Dependencies: {}
impl < 'a , T : Send > ParallelIterator for ParIterMut < 'a , T > { type Item = & 'a mut T ; # [cfg_attr (feature = "inline-more" , inline)] fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . map (| x | unsafe { x . as_mut () }) . drive_unindexed (consumer) } }
};
}
