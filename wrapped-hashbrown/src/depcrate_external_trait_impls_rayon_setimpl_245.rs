// Generated macro for impl_245 (impl)
macro_rules! Depcrate_external_trait_impls_rayon_setimpl_245 {
() => {
// Module: crate::external_trait_impls::rayon::set
// Provides: {"impl_245"}
// Dependencies: {}
impl < 'a , T , S , A > ParallelIterator for ParUnion < 'a , T , S , A > where T : Eq + Hash + Sync , S : BuildHasher + Sync , A : Allocator + Sync , { type Item = & 'a T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let (smaller , larger) = if self . a . len () <= self . b . len () { (self . a , self . b) } else { (self . b , self . a) } ; larger . into_par_iter () . chain (smaller . par_difference (larger)) . drive_unindexed (consumer) } }
};
}
