// Generated macro for impl_144 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_144 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_144"}
// Dependencies: {}
impl < 'a , T : Send > IntoParallelIterator for & 'a mut VecDeque < T > { type Item = & 'a mut T ; type Iter = IterMut < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { let (a , b) = self . as_mut_slices () ; IterMut { inner : a . into_par_iter () . chain (b) , } } }
};
}
