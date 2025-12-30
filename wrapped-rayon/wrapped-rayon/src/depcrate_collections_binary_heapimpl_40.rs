// Generated macro for impl_40 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_40 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_40"}
// Dependencies: {}
impl < T : Send > IntoParallelIterator for BinaryHeap < T > { type Item = T ; type Iter = IntoIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoIter { inner : Vec :: from (self) . into_par_iter () , } } }
};
}
