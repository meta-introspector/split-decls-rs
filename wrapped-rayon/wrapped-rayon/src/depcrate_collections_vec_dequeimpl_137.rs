// Generated macro for impl_137 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_137 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_137"}
// Dependencies: {}
impl < T : Send > IntoParallelIterator for VecDeque < T > { type Item = T ; type Iter = IntoIter < T > ; fn into_par_iter (self) -> Self :: Iter { let inner = Vec :: from (self) . into_par_iter () ; IntoIter { inner } } }
};
}
