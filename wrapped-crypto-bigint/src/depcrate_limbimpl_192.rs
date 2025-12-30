// Generated macro for impl_192 (impl)
macro_rules! Depcrate_limbimpl_192 {
() => {
// Module: crate::limb
// Provides: {"impl_192"}
// Dependencies: {}
impl ConditionallySelectable for Limb { # [inline] fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self (Word :: conditional_select (& a . 0 , & b . 0 , choice)) } }
};
}
