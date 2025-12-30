// Generated macro for impl_239 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_setimpl_239 {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"impl_239"}
// Dependencies: {}
impl < 'a , T , S , A > ParallelIterator for ParDifference < 'a , T , S , A > where T : Eq + Hash + Sync , S : BuildHasher + Sync , A : Allocator + Sync , { type Item = & 'a T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { self . a . into_par_iter () . filter (| & x | ! self . b . contains (x)) . drive_unindexed (consumer) } }
};
}
