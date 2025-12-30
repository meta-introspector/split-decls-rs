// Generated macro for impl_68 (impl)
macro_rules! Depcrate_montyimpl_68 {
() => {
// Module: crate::monty
// Provides: {"impl_68"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > From < & MontyFieldElement < MOD , LIMBS > > for MontyFieldBytes < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , Uint < LIMBS > : ArrayEncoding , { fn from (fe : & MontyFieldElement < MOD , LIMBS >) -> Self { fe . to_bytes () } }
};
}
