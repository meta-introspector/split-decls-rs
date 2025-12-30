// Generated macro for impl_564 (impl)
macro_rules! Depcrate_iter_extendimpl_564 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_564"}
// Dependencies: {}
# [doc = " Extends a deque with items from a parallel iterator."] impl < T > ParallelExtend < T > for VecDeque < T > where T : Send , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = T > , { extend_reserved ! (self , par_iter) ; } }
};
}
