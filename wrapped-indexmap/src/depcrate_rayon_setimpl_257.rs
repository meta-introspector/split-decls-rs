// Generated macro for impl_257 (impl)
macro_rules! Depcrate_rayon_setimpl_257 {
() => {
// Module: crate::rayon::set
// Provides: {"impl_257"}
// Dependencies: {}
impl < 'a , T , S1 , S2 > ParallelIterator for ParUnion < 'a , T , S1 , S2 > where T : Hash + Eq + Sync , S1 : BuildHasher + Sync , S2 : BuildHasher + Sync , { type Item = & 'a T ; fn drive_unindexed < C > (self , consumer : C) -> C :: Result where C : UnindexedConsumer < Self :: Item > , { let Self { set1 , set2 } = self ; set1 . par_iter () . chain (set2 . par_difference (set1)) . drive_unindexed (consumer) } }
};
}
