// Generated macro for impl_751 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_751 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_751"}
// Dependencies: {}
# [doc = " Collects items from a parallel iterator into an atomically-reference-counted slice."] impl < T > FromParallelIterator < T > for Arc < [T] > where T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { Vec :: from_par_iter (par_iter) . into () } }
};
}
