// Generated macro for impl_322 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_322 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_322"}
// Dependencies: {}
# [stable (feature = "std_collections_from_array" , since = "1.56.0")] impl < T : Ord , const N : usize > From < [T ; N] > for BinaryHeap < T > { # [doc = " ```"] # [doc = " use std::collections::BinaryHeap;"] # [doc = ""] # [doc = " let mut h1 = BinaryHeap::from([1, 4, 2, 3]);"] # [doc = " let mut h2: BinaryHeap<_> = [1, 4, 2, 3].into();"] # [doc = " while let Some((a, b)) = h1.pop().zip(h2.pop()) {"] # [doc = "     assert_eq!(a, b);"] # [doc = " }"] # [doc = " ```"] fn from (arr : [T ; N]) -> Self { Self :: from_iter (arr) } }
};
}
