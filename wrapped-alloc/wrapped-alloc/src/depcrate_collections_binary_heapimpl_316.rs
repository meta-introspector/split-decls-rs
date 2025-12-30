// Generated macro for impl_316 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_316 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_316"}
// Dependencies: {}
# [unstable (feature = "binary_heap_drain_sorted" , issue = "59278")] impl < 'a , T : Ord , A : Allocator > Drop for DrainSorted < 'a , T , A > { # [doc = " Removes heap elements in heap order."] fn drop (& mut self) { struct DropGuard < 'r , 'a , T : Ord , A : Allocator > (& 'r mut DrainSorted < 'a , T , A >) ; impl < 'r , 'a , T : Ord , A : Allocator > Drop for DropGuard < 'r , 'a , T , A > { fn drop (& mut self) { while self . 0 . inner . pop () . is_some () { } } } while let Some (item) = self . inner . pop () { let guard = DropGuard (self) ; drop (item) ; mem :: forget (guard) ; } } }
};
}
