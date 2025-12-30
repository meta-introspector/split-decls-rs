// Generated macro for impl_36 (impl)
macro_rules! Depcrate_montyimpl_36 {
() => {
// Module: crate::monty
// Provides: {"impl_36"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > AddAssign < Self > for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { # [inline] fn add_assign (& mut self , other : MontyFieldElement < MOD , LIMBS >) { * self = * self + other ; } }
};
}
