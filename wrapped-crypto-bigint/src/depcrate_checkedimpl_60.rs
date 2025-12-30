// Generated macro for impl_60 (impl)
macro_rules! Depcrate_checkedimpl_60 {
() => {
// Module: crate::checked
// Provides: {"impl_60"}
// Dependencies: {}
impl < T : ConditionallySelectable > ConditionallySelectable for Checked < T > { # [inline] fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self (CtOption :: conditional_select (& a . 0 , & b . 0 , choice)) } }
};
}
