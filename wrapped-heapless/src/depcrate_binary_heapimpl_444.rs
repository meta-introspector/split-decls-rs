// Generated macro for impl_444 (impl)
macro_rules! Depcrate_binary_heapimpl_444 {
() => {
// Module: crate::binary_heap
// Provides: {"impl_444"}
// Dependencies: {}
impl < T , K , S > Drop for PeekMutInner < '_ , T , K , S > where T : Ord , K : Kind , S : VecStorage < T > + ? Sized , { fn drop (& mut self) { if self . sift { self . heap . sift_down_to_bottom (0) ; } } }
};
}
