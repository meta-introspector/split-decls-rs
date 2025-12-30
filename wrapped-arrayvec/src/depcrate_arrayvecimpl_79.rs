// Generated macro for impl_79 (impl)
macro_rules! Depcrate_arrayvecimpl_79 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_79"}
// Dependencies: {}
impl < T , const CAP : usize > Deref for ArrayVec < T , CAP > { type Target = [T] ; # [inline] fn deref (& self) -> & Self :: Target { self . as_slice () } }
};
}
