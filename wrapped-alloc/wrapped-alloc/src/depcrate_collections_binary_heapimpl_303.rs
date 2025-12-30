// Generated macro for impl_303 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_303 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_303"}
// Dependencies: {}
impl < T , A : Allocator > IntoIterSorted < T , A > { # [doc = " Returns a reference to the underlying allocator."] # [unstable (feature = "allocator_api" , issue = "32838")] pub fn allocator (& self) -> & A { self . inner . allocator () } }
};
}
