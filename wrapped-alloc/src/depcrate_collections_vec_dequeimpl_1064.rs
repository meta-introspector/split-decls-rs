// Generated macro for impl_1064 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1064 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1064"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T , A : Allocator > IntoIterator for & 'a mut VecDeque < T , A > { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> IterMut < 'a , T > { self . iter_mut () } }
};
}
