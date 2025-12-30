// Generated macro for impl_291 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_291 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_291"}
// Dependencies: {}
impl < T , A : Allocator > IntoIter < T , A > { # [doc = " Returns a reference to the underlying allocator."] # [unstable (feature = "allocator_api" , issue = "32838")] pub fn allocator (& self) -> & A { self . iter . allocator () } }
};
}
