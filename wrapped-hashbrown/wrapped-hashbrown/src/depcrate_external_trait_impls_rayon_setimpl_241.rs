// Generated macro for impl_241 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_setimpl_241 {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"impl_241"}
// Dependencies: {}
impl < 'a , T , S , A > ParallelIterator for ParSymmetricDifference < 'a , T , S , A > where T : Eq + Hash + Sync , S : BuildHasher + Sync , A : Allocator + Sync , { type Item = & 'a T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . a . par_difference (self . b) . chain (self . b . par_difference (self . a)) . drive_unindexed (consumer) } }
};
}
