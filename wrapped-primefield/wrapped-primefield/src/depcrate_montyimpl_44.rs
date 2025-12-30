// Generated macro for impl_44 (impl)
macro_rules! Depcrate_montyimpl_44 {
() => {
// Module: crate::monty
// Provides: {"impl_44"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > Sum for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn sum < I : Iterator < Item = Self > > (iter : I) -> Self { iter . reduce (Add :: add) . unwrap_or (Self :: ZERO) } }
};
}
