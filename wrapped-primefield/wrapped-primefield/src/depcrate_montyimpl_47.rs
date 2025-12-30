// Generated macro for impl_47 (impl)
macro_rules! Depcrate_montyimpl_47 {
() => {
// Module: crate::monty
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'a , MOD : MontyFieldParams < LIMBS > , const LIMBS : usize > Product < & 'a MontyFieldElement < MOD , LIMBS > > for MontyFieldElement < MOD , LIMBS > { fn product < I : Iterator < Item = & 'a Self > > (iter : I) -> Self { iter . copied () . product () } }
};
}
