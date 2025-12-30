// Generated macro for DrainSorted (struct)
macro_rules! Depcrate_collections_binary_heapDrainSorted {
() => {
// Module: crate::collections::binary_heap
// Provides: {"DrainSorted"}
// Dependencies: {}
# [doc = " A draining iterator over the elements of a `BinaryHeap`."] # [doc = ""] # [doc = " This `struct` is created by [`BinaryHeap::drain_sorted()`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`drain_sorted`]: BinaryHeap::drain_sorted"] # [unstable (feature = "binary_heap_drain_sorted" , issue = "59278")] # [derive (Debug)] pub struct DrainSorted < 'a , T : Ord , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { inner : & 'a mut BinaryHeap < T , A > , }
};
}
