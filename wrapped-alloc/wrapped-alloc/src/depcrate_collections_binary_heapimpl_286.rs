// Generated macro for impl_286 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_286 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_286"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T > Iterator for Iter < 'a , T > { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } # [inline] fn last (self) -> Option < & 'a T > { self . iter . last () } }
};
}
