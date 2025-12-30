// Generated macro for impl_65 (impl)
macro_rules! Depcrate_montyimpl_65 {
() => {
// Module: crate::monty
// Provides: {"impl_65"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > From < u64 > for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { # [inline] fn from (n : u64) -> MontyFieldElement < MOD , LIMBS > { Self :: from_u64 (n) } }
};
}
