// Generated macro for IntoIter (struct)
macro_rules! Depcrate_collections_binary_heapIntoIter {
() => {
// Module: crate::collections::binary_heap
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over the elements of a `BinaryHeap`."] # [doc = ""] # [doc = " This `struct` is created by [`BinaryHeap::into_iter()`]"] # [doc = " (provided by the [`IntoIterator`] trait). See its documentation for more."] # [doc = ""] # [doc = " [`into_iter`]: BinaryHeap::into_iter"] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (Clone)] pub struct IntoIter < T , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { iter : vec :: IntoIter < T , A > , }
};
}
