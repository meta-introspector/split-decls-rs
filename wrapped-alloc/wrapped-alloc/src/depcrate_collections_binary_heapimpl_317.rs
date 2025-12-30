// Generated macro for impl_317 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_317 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_317"}
// Dependencies: {}
# [unstable (feature = "binary_heap_drain_sorted" , issue = "59278")] impl < T : Ord , A : Allocator > Iterator for DrainSorted < '_ , T , A > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . inner . pop () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let exact = self . inner . len () ; (exact , Some (exact)) } }
};
}
