// Generated macro for impl_235 (impl)
macro_rules! Depcrate_non_zeroimpl_235 {
() => {
// Module: crate::non_zero
// Provides: {"impl_235"}
// Dependencies: {}
impl < T > ConditionallySelectable for NonZero < T > where T : ConditionallySelectable , { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self (T :: conditional_select (& a . 0 , & b . 0 , choice)) } }
};
}
