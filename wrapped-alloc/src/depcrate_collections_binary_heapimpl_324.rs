// Generated macro for impl_324 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_324 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_324"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Ord > FromIterator < T > for BinaryHeap < T > { fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> BinaryHeap < T > { BinaryHeap :: from (iter . into_iter () . collect :: < Vec < _ > > ()) } }
};
}
