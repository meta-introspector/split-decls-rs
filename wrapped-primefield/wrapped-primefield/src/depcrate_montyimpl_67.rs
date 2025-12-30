// Generated macro for impl_67 (impl)
macro_rules! Depcrate_montyimpl_67 {
() => {
// Module: crate::monty
// Provides: {"impl_67"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > From < MontyFieldElement < MOD , LIMBS > > for MontyFieldBytes < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , Uint < LIMBS > : ArrayEncoding , { fn from (fe : MontyFieldElement < MOD , LIMBS >) -> Self { MontyFieldBytes :: < MOD , LIMBS > :: from (& fe) } }
};
}
