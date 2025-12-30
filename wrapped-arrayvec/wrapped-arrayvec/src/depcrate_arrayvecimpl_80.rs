// Generated macro for impl_80 (impl)
macro_rules! Depcrate_arrayvecimpl_80 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_80"}
// Dependencies: {}
impl < T , const CAP : usize > DerefMut for ArrayVec < T , CAP > { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { self . as_mut_slice () } }
};
}
