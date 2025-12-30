// Generated macro for impl_550 (impl)
macro_rules! Depcrate_iter_extendimpl_550 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_550"}
// Dependencies: {}
# [doc = " Extends an OS-string with string slices from a parallel iterator."] impl < 'a > ParallelExtend < & 'a OsStr > for OsString { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = & 'a OsStr > , { extend_reserved ! (self , par_iter , osstring_len) ; } }
};
}
