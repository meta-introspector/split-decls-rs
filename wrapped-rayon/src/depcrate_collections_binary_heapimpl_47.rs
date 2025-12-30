// Generated macro for impl_47 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_47 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a , T : Ord + Send > ParallelDrainFull for & 'a mut BinaryHeap < T > { type Iter = Drain < 'a , T > ; type Item = T ; fn par_drain (self) -> Self :: Iter { Drain { heap : self } } }
};
}
