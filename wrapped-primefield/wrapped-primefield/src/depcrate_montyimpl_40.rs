// Generated macro for impl_40 (impl)
macro_rules! Depcrate_montyimpl_40 {
() => {
// Module: crate::monty
// Provides: {"impl_40"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > MulAssign < & Self > for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { # [inline] fn mul_assign (& mut self , other : & MontyFieldElement < MOD , LIMBS >) { * self = * self * other ; } }
};
}
