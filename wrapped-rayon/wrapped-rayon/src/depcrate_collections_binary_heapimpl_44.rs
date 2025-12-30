// Generated macro for impl_44 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_44 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'a , T : Sync > IntoParallelIterator for & 'a BinaryHeap < T > { type Item = & 'a T ; type Iter = Iter < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { Iter { inner : self . as_slice () . into_par_iter () , } } }
};
}
