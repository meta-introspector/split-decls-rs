// Generated macro for impl_95 (impl)
macro_rules! Depcrate_boxedimpl_95 {
() => {
// Module: crate::boxed
// Provides: {"impl_95"}
// Dependencies: {}
impl < I : ExactSizeIterator + ? Sized , A : Allocator > ExactSizeIterator for Box < I , A > { # [inline (always)] fn len (& self) -> usize { (* * self) . len () } }
};
}
