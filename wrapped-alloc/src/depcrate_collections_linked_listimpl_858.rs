// Generated macro for impl_858 (impl)
macro_rules! Depcrate_collections_linked_listimpl_858 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_858"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T , A : Allocator > IntoIterator for & 'a LinkedList < T , A > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
