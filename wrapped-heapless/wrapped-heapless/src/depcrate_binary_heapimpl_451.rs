// Generated macro for impl_451 (impl)
macro_rules! Depcrate_binary_heapimpl_451 {
() => {
// Module: crate::binary_heap
// Provides: {"impl_451"}
// Dependencies: {}
impl < T , K , S > fmt :: Debug for BinaryHeapInner < T , K , S > where K : Kind , T : Ord + fmt :: Debug , S : VecStorage < T > + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
