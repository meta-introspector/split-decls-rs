// Generated macro for impl_749 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_749 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_749"}
// Dependencies: {}
# [doc = " Collects items from a parallel iterator into a boxed slice."] impl < T > FromParallelIterator < T > for Box < [T] > where T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { Vec :: from_par_iter (par_iter) . into () } }
};
}
