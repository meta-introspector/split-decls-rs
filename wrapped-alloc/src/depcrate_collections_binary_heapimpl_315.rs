// Generated macro for impl_315 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_315 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_315"}
// Dependencies: {}
impl < 'a , T : Ord , A : Allocator > DrainSorted < 'a , T , A > { # [doc = " Returns a reference to the underlying allocator."] # [unstable (feature = "allocator_api" , issue = "32838")] pub fn allocator (& self) -> & A { self . inner . allocator () } }
};
}
