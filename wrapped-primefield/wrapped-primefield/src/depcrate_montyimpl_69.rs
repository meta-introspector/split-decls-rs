// Generated macro for impl_69 (impl)
macro_rules! Depcrate_montyimpl_69 {
() => {
// Module: crate::monty
// Provides: {"impl_69"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > From < MontyFieldElement < MOD , LIMBS > > for Uint < LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn from (fe : MontyFieldElement < MOD , LIMBS >) -> Uint < LIMBS > { Uint :: from (& fe) } }
};
}
