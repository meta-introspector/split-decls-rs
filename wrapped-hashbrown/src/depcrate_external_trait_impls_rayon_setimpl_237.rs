// Generated macro for impl_237 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_setimpl_237 {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"impl_237"}
// Dependencies: {}
impl < 'a , T : Sync > ParallelIterator for ParIter < 'a , T > { type Item = & 'a T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . inner . drive_unindexed (consumer) } }
};
}
