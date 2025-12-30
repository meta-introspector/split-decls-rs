// Generated macro for impl_326 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_326 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_326"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T , A : Allocator > IntoIterator for & 'a BinaryHeap < T , A > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
