// Generated macro for impl_88 (impl)
macro_rules! Depcrate_arrayvecimpl_88 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_88"}
// Dependencies: {}
impl < T , const CAP : usize > IntoIter < T , CAP > { # [doc = " Returns the remaining items of this iterator as a slice."] pub fn as_slice (& self) -> & [T] { & self . v [self . index ..] } # [doc = " Returns the remaining items of this iterator as a mutable slice."] pub fn as_mut_slice (& mut self) -> & mut [T] { & mut self . v [self . index ..] } }
};
}
