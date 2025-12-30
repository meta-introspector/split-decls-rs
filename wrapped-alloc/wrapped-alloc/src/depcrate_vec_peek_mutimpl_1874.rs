// Generated macro for impl_1874 (impl)
macro_rules! Depcrate_vec_peek_mutimpl_1874 {
() => {
// Module: crate::vec::peek_mut
// Provides: {"impl_1874"}
// Dependencies: {}
# [unstable (feature = "vec_peek_mut" , issue = "122742")] impl < 'a , T > Deref for PeekMut < 'a , T > { type Target = T ; fn deref (& self) -> & Self :: Target { unsafe { self . vec . get_unchecked (self . vec . len () - 1) } } }
};
}
