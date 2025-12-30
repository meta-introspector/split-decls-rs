// Generated macro for impl_63 (impl)
macro_rules! Depcrate_montyimpl_63 {
() => {
// Module: crate::monty
// Provides: {"impl_63"}
// Dependencies: {}
impl < MOD : MontyFieldParams < LIMBS > , const LIMBS : usize > PartialEq for MontyFieldElement < MOD , LIMBS > { fn eq (& self , rhs : & Self) -> bool { self . inner . ct_eq (& (rhs . inner)) . into () } }
};
}
