// Generated macro for impl_532 (impl)
macro_rules! Depcrate_iter_extendimpl_532 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_532"}
// Dependencies: {}
# [doc = " Extends a binary heap with copied items from a parallel iterator."] impl < 'a , T > ParallelExtend < & 'a T > for BinaryHeap < T > where T : 'a + Copy + Ord + Send + Sync , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a T > , { extend_reserved ! (self , par_iter) ; } }
};
}
