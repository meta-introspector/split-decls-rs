// Generated macro for impl_325 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_325 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_325"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > IntoIterator for BinaryHeap < T , A > { type Item = T ; type IntoIter = IntoIter < T , A > ; # [doc = " Creates a consuming iterator, that is, one that moves each value out of"] # [doc = " the binary heap in arbitrary order. The binary heap cannot be used"] # [doc = " after calling this."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::BinaryHeap;"] # [doc = " let heap = BinaryHeap::from([1, 2, 3, 4]);"] # [doc = ""] # [doc = " // Print 1, 2, 3, 4 in arbitrary order"] # [doc = " for x in heap.into_iter() {"] # [doc = "     // x has type i32, not &i32"] # [doc = "     println!(\"{x}\");"] # [doc = " }"] # [doc = " ```"] fn into_iter (self) -> IntoIter < T , A > { IntoIter { iter : self . data . into_iter () } } }
};
}
