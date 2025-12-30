// Generated macro for impl_536 (impl)
macro_rules! Depcrate_iter_extendimpl_536 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_536"}
// Dependencies: {}
# [doc = " Extends a B-tree set with copied items from a parallel iterator."] impl < 'a , T > ParallelExtend < & 'a T > for BTreeSet < T > where T : 'a + Copy + Ord + Send + Sync , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a T > , { extend ! (self , par_iter) ; } }
};
}
