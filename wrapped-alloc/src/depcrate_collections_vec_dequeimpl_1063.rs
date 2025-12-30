// Generated macro for impl_1063 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1063 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1063"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T , A : Allocator > IntoIterator for & 'a VecDeque < T , A > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
