// Generated macro for impl_323 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_323 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_323"}
// Dependencies: {}
# [stable (feature = "binary_heap_extras_15" , since = "1.5.0")] impl < T , A : Allocator > From < BinaryHeap < T , A > > for Vec < T , A > { # [doc = " Converts a `BinaryHeap<T>` into a `Vec<T>`."] # [doc = ""] # [doc = " This conversion requires no data movement or allocation, and has"] # [doc = " constant time complexity."] fn from (heap : BinaryHeap < T , A >) -> Vec < T , A > { heap . data } }
};
}
