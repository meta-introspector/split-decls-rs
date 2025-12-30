// Generated macro for impl_43 (impl)
macro_rules! Depcrate_montyimpl_43 {
() => {
// Module: crate::monty
// Provides: {"impl_43"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > Neg for & MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { type Output = MontyFieldElement < MOD , LIMBS > ; # [inline] fn neg (self) -> MontyFieldElement < MOD , LIMBS > { < MontyFieldElement < MOD , LIMBS > > :: neg (self) } }
};
}
