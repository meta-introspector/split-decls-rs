// Generated macro for impl_26 (impl)
macro_rules! Depcrate_boxedimpl_26 {
() => {
// Module: crate::boxed
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'a , I : ExactSizeIterator + ? Sized > ExactSizeIterator for Box < 'a , I > { fn len (& self) -> usize { (* * self) . len () } }
};
}
