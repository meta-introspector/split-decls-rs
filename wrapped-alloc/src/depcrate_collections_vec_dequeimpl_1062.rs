// Generated macro for impl_1062 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1062 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1062"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > IntoIterator for VecDeque < T , A > { type Item = T ; type IntoIter = IntoIter < T , A > ; # [doc = " Consumes the deque into a front-to-back iterator yielding elements by"] # [doc = " value."] fn into_iter (self) -> IntoIter < T , A > { IntoIter :: new (self) } }
};
}
