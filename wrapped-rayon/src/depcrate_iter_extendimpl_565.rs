// Generated macro for impl_565 (impl)
macro_rules! Depcrate_iter_extendimpl_565 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_565"}
// Dependencies: {}
# [doc = " Extends a deque with copied items from a parallel iterator."] impl < 'a , T > ParallelExtend < & 'a T > for VecDeque < T > where T : 'a + Copy + Send + Sync , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a T > , { extend_reserved ! (self , par_iter) ; } }
};
}
