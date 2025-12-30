// Generated macro for impl_857 (impl)
macro_rules! Depcrate_collections_linked_listimpl_857 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_857"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > IntoIterator for LinkedList < T , A > { type Item = T ; type IntoIter = IntoIter < T , A > ; # [doc = " Consumes the list into an iterator yielding elements by value."] # [inline] fn into_iter (self) -> IntoIter < T , A > { IntoIter { list : self } } }
};
}
