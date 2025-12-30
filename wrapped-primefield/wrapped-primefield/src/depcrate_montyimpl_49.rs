// Generated macro for impl_49 (impl)
macro_rules! Depcrate_montyimpl_49 {
() => {
// Module: crate::monty
// Provides: {"impl_49"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > Reduce < Uint < LIMBS > > for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { # [inline] fn reduce (w : & Uint < LIMBS >) -> Self { Self :: from_uint_reduced (w) } }
};
}
