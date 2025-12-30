// Generated macro for impl_489 (impl)
macro_rules! Depcrate_wrappingimpl_489 {
() => {
// Module: crate::wrapping
// Provides: {"impl_489"}
// Dependencies: {}
impl < T : ConditionallySelectable > ConditionallySelectable for Wrapping < T > { # [inline] fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Wrapping (T :: conditional_select (& a . 0 , & b . 0 , choice)) } }
};
}
