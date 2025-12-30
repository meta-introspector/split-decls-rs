// Generated macro for impl_267 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_267 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_267"}
// Dependencies: {}
# [stable (feature = "binary_heap_peek_mut" , since = "1.12.0")] impl < T : Ord , A : Allocator > Drop for PeekMut < '_ , T , A > { fn drop (& mut self) { if let Some (original_len) = self . original_len { unsafe { self . heap . data . set_len (original_len . get ()) } ; unsafe { self . heap . sift_down (0) } ; } } }
};
}
