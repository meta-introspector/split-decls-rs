// Generated macro for impl_276 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_276 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_276"}
// Dependencies: {}
impl < T : Ord > BinaryHeap < T > { # [doc = " Creates an empty `BinaryHeap` as a max-heap."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::BinaryHeap;"] # [doc = " let mut heap = BinaryHeap::new();"] # [doc = " heap.push(4);"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_const_stable (feature = "const_binary_heap_constructor" , since = "1.80.0")] # [must_use] pub const fn new () -> BinaryHeap < T > { BinaryHeap { data : vec ! [] } } # [doc = " Creates an empty `BinaryHeap` with at least the specified capacity."] # [doc = ""] # [doc = " The binary heap will be able to hold at least `capacity` elements without"] # [doc = " reallocating. This method is allowed to allocate for more elements than"] # [doc = " `capacity`. If `capacity` is zero, the binary heap will not allocate."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::BinaryHeap;"] # [doc = " let mut heap = BinaryHeap::with_capacity(10);"] # [doc = " heap.push(4);"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [must_use] pub fn with_capacity (capacity : usize) -> BinaryHeap < T > { BinaryHeap { data : Vec :: with_capacity (capacity) } } }
};
}
