// Generated macro for impl_748 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_748 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_748"}
// Dependencies: {}
# [doc = " Collects items from a parallel iterator into a vector."] impl < T > FromParallelIterator < T > for Vec < T > where T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { collect_extended (par_iter) } }
};
}
