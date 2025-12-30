// Generated macro for impl_293 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_293 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_293"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > Iterator for IntoIter < T , A > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
