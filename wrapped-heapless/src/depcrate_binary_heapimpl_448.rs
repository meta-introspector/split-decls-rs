// Generated macro for impl_448 (impl)
macro_rules! Depcrate_binary_heapimpl_448 {
() => {
// Module: crate::binary_heap
// Provides: {"impl_448"}
// Dependencies: {}
impl < T > Drop for Hole < '_ , T > { # [inline] fn drop (& mut self) { unsafe { let pos = self . pos ; ptr :: write (self . data . get_unchecked_mut (pos) , ptr :: read (& * self . elt)) ; } } }
};
}
