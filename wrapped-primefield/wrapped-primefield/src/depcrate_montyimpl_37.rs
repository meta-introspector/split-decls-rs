// Generated macro for impl_37 (impl)
macro_rules! Depcrate_montyimpl_37 {
() => {
// Module: crate::monty
// Provides: {"impl_37"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > AddAssign < & Self > for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { # [inline] fn add_assign (& mut self , other : & MontyFieldElement < MOD , LIMBS >) { * self = * self + other ; } }
};
}
