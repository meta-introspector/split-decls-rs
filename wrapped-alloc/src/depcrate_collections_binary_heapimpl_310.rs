// Generated macro for impl_310 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_310 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_310"}
// Dependencies: {}
# [stable (feature = "drain" , since = "1.6.0")] impl < T , A : Allocator > Iterator for Drain < '_ , T , A > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
