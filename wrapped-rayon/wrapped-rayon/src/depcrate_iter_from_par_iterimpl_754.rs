// Generated macro for impl_754 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_754 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_754"}
// Dependencies: {}
# [doc = " Collects items from a parallel iterator into a freshly allocated"] # [doc = " linked list."] impl < T > FromParallelIterator < T > for LinkedList < T > where T : Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { collect_extended (par_iter) } }
};
}
