// Generated macro for impl_750 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_750 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_750"}
// Dependencies: {}
# [doc = " Collects items from a parallel iterator into a reference-counted slice."] impl < T > FromParallelIterator < T > for Rc < [T] > where T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { Vec :: from_par_iter (par_iter) . into () } }
};
}
