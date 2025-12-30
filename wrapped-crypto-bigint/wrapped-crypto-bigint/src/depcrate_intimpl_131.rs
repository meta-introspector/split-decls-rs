// Generated macro for impl_131 (impl)
macro_rules! Depcrate_intimpl_131 {
() => {
// Module: crate::int
// Provides: {"impl_131"}
// Dependencies: {}
impl < const LIMBS : usize > ConditionallySelectable for Int < LIMBS > { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self (Uint :: conditional_select (& a . 0 , & b . 0 , choice)) } }
};
}
