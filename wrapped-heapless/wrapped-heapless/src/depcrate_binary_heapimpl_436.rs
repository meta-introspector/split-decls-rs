// Generated macro for impl_436 (impl)
macro_rules! Depcrate_binary_heapimpl_436 {
() => {
// Module: crate::binary_heap
// Provides: {"impl_436"}
// Dependencies: {}
impl < T , K , const N : usize > BinaryHeap < T , K , N > { # [doc = " Returns the underlying `Vec<T,N>`. Order is arbitrary and time is *O*(1)."] pub fn into_vec (self) -> Vec < T , N , usize > { self . data } }
};
}
