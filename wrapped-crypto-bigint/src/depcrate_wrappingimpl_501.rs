// Generated macro for impl_501 (impl)
macro_rules! Depcrate_wrappingimpl_501 {
() => {
// Module: crate::wrapping
// Provides: {"impl_501"}
// Dependencies: {}
impl < T : num_traits :: Zero + WrappingAdd > num_traits :: Zero for Wrapping < T > { # [inline] fn zero () -> Self { Wrapping (T :: zero ()) } # [inline] fn is_zero (& self) -> bool { self . 0 . is_zero () } }
};
}
