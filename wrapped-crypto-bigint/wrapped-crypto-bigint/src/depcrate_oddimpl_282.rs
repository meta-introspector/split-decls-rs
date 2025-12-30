// Generated macro for impl_282 (impl)
macro_rules! Depcrate_oddimpl_282 {
() => {
// Module: crate::odd
// Provides: {"impl_282"}
// Dependencies: {}
impl < T > ConditionallySelectable for Odd < T > where T : ConditionallySelectable , { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self (T :: conditional_select (& a . 0 , & b . 0 , choice)) } }
};
}
