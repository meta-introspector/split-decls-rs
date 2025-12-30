// Generated macro for impl_281 (impl)
macro_rules! Depcrate_collections_binary_heapimpl_281 {
() => {
// Module: crate::collections::binary_heap
// Provides: {"impl_281"}
// Dependencies: {}
impl < T > Drop for Hole < '_ , T > { # [inline] fn drop (& mut self) { unsafe { let pos = self . pos ; ptr :: copy_nonoverlapping (& * self . elt , self . data . get_unchecked_mut (pos) , 1) ; } } }
};
}
