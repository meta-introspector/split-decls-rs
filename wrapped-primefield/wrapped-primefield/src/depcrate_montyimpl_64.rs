// Generated macro for impl_64 (impl)
macro_rules! Depcrate_montyimpl_64 {
() => {
// Module: crate::monty
// Provides: {"impl_64"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > From < u32 > for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { # [inline] fn from (n : u32) -> MontyFieldElement < MOD , LIMBS > { Self :: from_uint_reduced (& Uint :: from (n)) } }
};
}
