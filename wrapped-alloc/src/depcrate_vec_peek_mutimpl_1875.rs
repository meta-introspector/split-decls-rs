// Generated macro for impl_1875 (impl)
macro_rules! Depcrate_vec_peek_mutimpl_1875 {
() => {
// Module: crate::vec::peek_mut
// Provides: {"impl_1875"}
// Dependencies: {}
# [unstable (feature = "vec_peek_mut" , issue = "122742")] impl < 'a , T > DerefMut for PeekMut < 'a , T > { fn deref_mut (& mut self) -> & mut Self :: Target { let idx = self . vec . len () - 1 ; unsafe { self . vec . get_unchecked_mut (idx) } } }
};
}
