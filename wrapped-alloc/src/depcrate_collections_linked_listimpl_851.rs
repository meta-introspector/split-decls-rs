// Generated macro for impl_851 (impl)
macro_rules! Depcrate_collections_linked_listimpl_851 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_851"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > Iterator for IntoIter < T , A > { type Item = T ; # [inline] fn next (& mut self) -> Option < T > { self . list . pop_front () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . list . len , Some (self . list . len)) } }
};
}
