// Generated macro for impl_766 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_766 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_766"}
// Dependencies: {}
# [doc = " Collects OS-string slices from a parallel iterator into an OS-string."] impl < 'a > FromParallelIterator < & 'a OsStr > for OsString { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = & 'a OsStr > , { collect_extended (par_iter) } }
};
}
