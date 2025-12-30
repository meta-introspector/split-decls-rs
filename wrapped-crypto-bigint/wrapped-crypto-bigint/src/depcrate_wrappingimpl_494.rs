// Generated macro for impl_494 (impl)
macro_rules! Depcrate_wrappingimpl_494 {
() => {
// Module: crate::wrapping
// Provides: {"impl_494"}
// Dependencies: {}
impl < T : num_traits :: One + WrappingMul + PartialEq > num_traits :: One for Wrapping < T > { # [inline] fn one () -> Self { Wrapping (T :: one ()) } # [inline] fn is_one (& self) -> bool { self . 0 . is_one () } }
};
}
