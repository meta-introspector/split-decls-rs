// Generated macro for impl_551 (impl)
macro_rules! Depcrate_iter_extendimpl_551 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_551"}
// Dependencies: {}
# [doc = " Extends an OS-string with strings from a parallel iterator."] impl ParallelExtend < OsString > for OsString { fn par_extend < I > (& mut self , par_iter : I) where I : IntoParallelIterator < Item = OsString > , { extend_reserved ! (self , par_iter , osstring_len) ; } }
};
}
