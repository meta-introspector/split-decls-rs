// Generated macro for impl_51 (impl)
macro_rules! Depcrate_montyimpl_51 {
() => {
// Module: crate::monty
// Provides: {"impl_51"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > ConditionallySelectable for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self { inner : MontyForm :: conditional_select (& a . inner , & b . inner , choice) , } } }
};
}
