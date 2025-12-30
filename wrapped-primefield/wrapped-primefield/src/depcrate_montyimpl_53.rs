// Generated macro for impl_53 (impl)
macro_rules! Depcrate_montyimpl_53 {
() => {
// Module: crate::monty
// Provides: {"impl_53"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > ConstantTimeGreater for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn ct_gt (& self , other : & Self) -> Choice { self . inner . retrieve () . ct_gt (& other . inner . retrieve ()) } }
};
}
