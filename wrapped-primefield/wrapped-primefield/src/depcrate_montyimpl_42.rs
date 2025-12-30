// Generated macro for impl_42 (impl)
macro_rules! Depcrate_montyimpl_42 {
() => {
// Module: crate::monty
// Provides: {"impl_42"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > Neg for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { type Output = MontyFieldElement < MOD , LIMBS > ; # [inline] fn neg (self) -> MontyFieldElement < MOD , LIMBS > { < MontyFieldElement < MOD , LIMBS > > :: neg (& self) } }
};
}
