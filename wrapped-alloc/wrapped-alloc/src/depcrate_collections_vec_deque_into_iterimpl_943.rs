// Generated macro for impl_943 (impl)
macro_rules! Depcrate_collections_vec_deque_into_iterimpl_943 {
() => {
// Module: crate::collections::vec_deque::into_iter
// Provides: {"impl_943"}
// Dependencies: {}
impl < T , A : Allocator > IntoIter < T , A > { pub (super) fn new (inner : VecDeque < T , A >) -> Self { IntoIter { inner } } pub (super) fn into_vecdeque (self) -> VecDeque < T , A > { self . inner } }
};
}
