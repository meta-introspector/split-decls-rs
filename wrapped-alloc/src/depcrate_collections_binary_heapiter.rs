// Generated macro for Iter (struct)
macro_rules! Depcrate_collections_binary_heapIter {
() => {
// Module: crate::collections::binary_heap
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the elements of a `BinaryHeap`."] # [doc = ""] # [doc = " This `struct` is created by [`BinaryHeap::iter()`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`iter`]: BinaryHeap::iter"] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct Iter < 'a , T : 'a > { iter : slice :: Iter < 'a , T > , }
};
}
