// Generated macro for Drain (struct)
macro_rules! Depcrate_collections_binary_heapDrain {
() => {
// Module: crate::collections::binary_heap
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator over the elements of a `BinaryHeap`."] # [doc = ""] # [doc = " This `struct` is created by [`BinaryHeap::drain()`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`drain`]: BinaryHeap::drain"] # [stable (feature = "drain" , since = "1.6.0")] # [derive (Debug)] pub struct Drain < 'a , T : 'a , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { iter : vec :: Drain < 'a , T , A > , }
};
}
