// Generated macro for impl_52 (impl)
macro_rules! Depcrate_montyimpl_52 {
() => {
// Module: crate::monty
// Provides: {"impl_52"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > ConstantTimeEq for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn ct_eq (& self , other : & Self) -> Choice { self . inner . ct_eq (& other . inner) } }
};
}
