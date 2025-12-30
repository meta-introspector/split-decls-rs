// Generated macro for impl_253 (impl)
macro_rules! Depcrate_rayon_setimpl_253 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_253"}
// Dependencies: {}
impl < 'a , T , S1 , S2 > ParallelIterator for ParSymmetricDifference < 'a , T , S1 , S2 > where T : Hash + Eq + Sync , S1 : BuildHasher + Sync , S2 : BuildHasher + Sync , { type Item = & 'a T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let Self { set1 , set2 } = self ; set1 . par_difference (set2) . chain (set2 . par_difference (set1)) . drive_unindexed (consumer) } }
};
}
