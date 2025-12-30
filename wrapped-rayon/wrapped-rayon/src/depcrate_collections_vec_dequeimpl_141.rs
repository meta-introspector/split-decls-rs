// Generated macro for impl_141 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_141 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_141"}
// Dependencies: {}
impl < 'a , T : Sync > IntoParallelIterator for & 'a VecDeque < T > { type Item = & 'a T ; type Iter = Iter < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { let (a , b) = self . as_slices () ; Iter { inner : a . into_par_iter () . chain (b) , } } }
};
}
