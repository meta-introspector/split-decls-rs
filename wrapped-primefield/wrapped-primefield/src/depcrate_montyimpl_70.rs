// Generated macro for impl_70 (impl)
macro_rules! Depcrate_montyimpl_70 {
() => {
// Module: crate::monty
// Provides: {"impl_70"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > From < & MontyFieldElement < MOD , LIMBS > > for Uint < LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn from (fe : & MontyFieldElement < MOD , LIMBS >) -> Uint < LIMBS > { fe . to_canonical () } }
};
}
