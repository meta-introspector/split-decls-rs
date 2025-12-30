// Generated macro for impl_271 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_271 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_271"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : Clone , A : Allocator + Clone > Clone for BinaryHeap < T , A > { fn clone (& self) -> Self { BinaryHeap { data : self . data . clone () } } # [doc = " Overwrites the contents of `self` with a clone of the contents of `source`."] # [doc = ""] # [doc = " This method is preferred over simply assigning `source.clone()` to `self`,"] # [doc = " as it avoids reallocation if possible."] # [doc = ""] # [doc = " See [`Vec::clone_from()`] for more details."] fn clone_from (& mut self , source : & Self) { self . data . clone_from (& source . data) ; } }
};
}
