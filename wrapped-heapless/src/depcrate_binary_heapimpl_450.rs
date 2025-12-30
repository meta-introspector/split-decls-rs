// Generated macro for impl_450 (impl)
macro_rules! Depcrate_binary_heapimpl_450 {
() => {
// Module: crate::binary_heap
// Provides: {"impl_450"}
// Dependencies: {}
impl < T , K , const N : usize > Clone for BinaryHeap < T , K , N > where K : Kind , T : Ord + Clone , { fn clone (& self) -> Self { Self { _kind : self . _kind , data : self . data . clone () , } } }
};
}
