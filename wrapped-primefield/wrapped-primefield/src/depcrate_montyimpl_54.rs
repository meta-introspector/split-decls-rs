// Generated macro for impl_54 (impl)
macro_rules! Depcrate_montyimpl_54 {
() => {
// Module: crate::monty
// Provides: {"impl_54"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > ConstantTimeLess for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn ct_lt (& self , other : & Self) -> Choice { self . inner . retrieve () . ct_lt (& other . inner . retrieve ()) } }
};
}
