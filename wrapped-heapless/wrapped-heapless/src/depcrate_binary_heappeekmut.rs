// Generated macro for PeekMut (type)
macro_rules! Depcrate_binary_heapPeekMut {
() => {
// Module: crate::binary_heap
// Provides: {"PeekMut"}
// Dependencies: {}
# [doc = " Structure wrapping a mutable reference to the greatest item on a"] # [doc = " `BinaryHeap`."] # [doc = ""] # [doc = " This `struct` is created by [`BinaryHeap::peek_mut`]."] # [doc = " See its documentation for more."] pub type PeekMut < 'a , T , K , const N : usize > = PeekMutInner < 'a , T , K , OwnedVecStorage < T , N > > ;
};
}
