// Generated macro for impl_46 (impl)
macro_rules! Depcrate_montyimpl_46 {
() => {
// Module: crate::monty
// Provides: {"impl_46"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > Product for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn product < I : Iterator < Item = Self > > (iter : I) -> Self { iter . reduce (Mul :: mul) . unwrap_or (Self :: ONE) } }
};
}
