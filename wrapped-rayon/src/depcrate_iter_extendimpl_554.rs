// Generated macro for impl_554 (impl)
macro_rules! Depcrate_iter_extendimpl_554 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_554"}
// Dependencies: {}
# [doc = " Extends a string with copied characters from a parallel iterator."] impl < 'a > ParallelExtend < & 'a char > for String { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a char > , { self . par_extend (par_iter . into_par_iter () . copied ()) } }
};
}
