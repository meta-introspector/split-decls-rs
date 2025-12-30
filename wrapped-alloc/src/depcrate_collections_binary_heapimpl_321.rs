// Generated macro for impl_321 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_321 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_321"}
// Dependencies: {}
# [stable (feature = "binary_heap_extras_15" , since = "1.5.0")] impl < T : Ord , A : Allocator > From < Vec < T , A > > for BinaryHeap < T , A > { # [doc = " Converts a `Vec<T>` into a `BinaryHeap<T>`."] # [doc = ""] # [doc = " This conversion happens in-place, and has *O*(*n*) time complexity."] fn from (vec : Vec < T , A >) -> BinaryHeap < T , A > { let mut heap = BinaryHeap { data : vec } ; heap . rebuild () ; heap } }
};
}
