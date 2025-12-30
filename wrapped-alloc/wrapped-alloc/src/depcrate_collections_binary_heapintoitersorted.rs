// Generated macro for IntoIterSorted (struct)
macro_rules! Depcrate_collections_binary_heapIntoIterSorted {
() => {
// Module: crate::collections::binary_heap
// Provides: {"IntoIterSorted"}
// Dependencies: {}
# [must_use = "iterators are lazy and do nothing unless consumed"] # [unstable (feature = "binary_heap_into_iter_sorted" , issue = "59278")] # [derive (Clone , Debug)] pub struct IntoIterSorted < T , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { inner : BinaryHeap < T , A > , }
};
}
