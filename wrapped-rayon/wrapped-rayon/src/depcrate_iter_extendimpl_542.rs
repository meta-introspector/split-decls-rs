// Generated macro for impl_542 (impl)
macro_rules! Depcrate_iter_extendimpl_542 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_542"}
// Dependencies: {}
# [doc = " Extends a linked list with copied items from a parallel iterator."] impl < 'a , T > ParallelExtend < & 'a T > for LinkedList < T > where T : 'a + Copy + Send + Sync , { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a T > , { self . par_extend (par_iter . into_par_iter () . copied ()) } }
};
}
