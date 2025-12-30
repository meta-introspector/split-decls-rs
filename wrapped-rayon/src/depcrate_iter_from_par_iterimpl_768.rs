// Generated macro for impl_768 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_768 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_768"}
// Dependencies: {}
# [doc = " Collects OS-string slices from a parallel iterator into an OS-string."] impl < 'a > FromParallelIterator < Cow < 'a , OsStr > > for OsString { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = Cow < 'a , OsStr > > , { collect_extended (par_iter) } }
};
}
