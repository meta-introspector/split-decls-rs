// Generated macro for impl_66 (impl)
macro_rules! Depcrate_montyimpl_66 {
() => {
// Module: crate::monty
// Provides: {"impl_66"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > From < u128 > for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn from (n : u128) -> MontyFieldElement < MOD , LIMBS > { Self :: from_uint_reduced (& Uint :: from (n)) } }
};
}
