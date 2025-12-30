// Generated macro for impl_859 (impl)
macro_rules! Depcrate_collections_linked_listimpl_859 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_859"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T , A : Allocator > IntoIterator for & 'a mut LinkedList < T , A > { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> IterMut < 'a , T > { self . iter_mut () } }
};
}
