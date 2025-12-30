// Generated macro for impl_753 (impl)
macro_rules! Depcrate_iter_from_par_iterimpl_753 {
() => {
// Module: crate::iter::from_par_iter
// Provides: {"impl_753"}
// Dependencies: {}
# [doc = " Collects items from a parallel iterator into a binaryheap."] # [doc = " The heap-ordering is calculated serially after all items are collected."] impl < T > FromParallelIterator < T > for BinaryHeap < T > where T : Ord + Send , { fn from_par_iter < I > (par_iter : I) -> Self where I : IntoParallelIterator < Item = T > , { Vec :: from_par_iter (par_iter) . into () } }
};
}
