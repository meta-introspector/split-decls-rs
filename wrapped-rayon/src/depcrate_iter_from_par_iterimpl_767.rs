// Generated macro for impl_767 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_767 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_767"}
// Dependencies: {}
# [doc = " Collects OS-strings from a parallel iterator into one large OS-string."] impl FromParallelIterator < OsString > for OsString { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = OsString > , { collect_extended (par_iter) } }
};
}
