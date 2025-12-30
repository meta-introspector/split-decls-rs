// Generated macro for PeekMut (struct)
macro_rules! Depcrate_collections_binary_heapPeekMut {
() => {
// Module: crate::collections::binary_heap
// Provides: {"PeekMut"}
// Dependencies: {}
# [doc = " Structure wrapping a mutable reference to the greatest item on a"] # [doc = " `BinaryHeap`."] # [doc = ""] # [doc = " This `struct` is created by the [`peek_mut`] method on [`BinaryHeap`]. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`peek_mut`]: BinaryHeap::peek_mut"] # [stable (feature = "binary_heap_peek_mut" , since = "1.12.0")] pub struct PeekMut < 'a , T : 'a + Ord , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { heap : & 'a mut BinaryHeap < T , A > , original_len : Option < NonZero < usize > > , }
};
}
