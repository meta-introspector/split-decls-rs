// Generated macro for impl_535 (impl)
macro_rules! Depcrate_iter_extendimpl_535 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_535"}
// Dependencies: {}
# [doc = " Extends a B-tree set with items from a parallel iterator."] impl < T > ParallelExtend < T > for BTreeSet < T > where T : Ord + Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > , { extend ! (self , par_iter) ; } }
};
}
