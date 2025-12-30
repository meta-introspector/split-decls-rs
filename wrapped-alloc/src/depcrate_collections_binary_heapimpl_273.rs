// Generated macro for impl_273 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_273 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_273"}
// Dependencies: {}
# [stable (feature = "binaryheap_debug" , since = "1.4.0")] impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for BinaryHeap < T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
