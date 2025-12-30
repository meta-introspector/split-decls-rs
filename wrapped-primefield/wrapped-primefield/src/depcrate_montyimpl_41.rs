// Generated macro for impl_41 (impl)
macro_rules! Depcrate_montyimpl_41 {
() => {
// Module: crate::monty
// Provides: {"impl_41"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > MulAssign for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { # [inline] fn mul_assign (& mut self , other : MontyFieldElement < MOD , LIMBS >) { * self = * self * other ; } }
};
}
