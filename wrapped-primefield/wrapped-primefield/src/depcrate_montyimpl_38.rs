// Generated macro for impl_38 (impl)
macro_rules! Depcrate_montyimpl_38 {
() => {
// Module: crate::monty
// Provides: {"impl_38"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > SubAssign < Self > for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { # [inline] fn sub_assign (& mut self , other : MontyFieldElement < MOD , LIMBS >) { * self = * self - other ; } }
};
}
