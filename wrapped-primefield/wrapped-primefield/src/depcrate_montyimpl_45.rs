// Generated macro for impl_45 (impl)
macro_rules! Depcrate_montyimpl_45 {
() => {
// Module: crate::monty
// Provides: {"impl_45"}
// Dependencies: {}
impl < 'a , MOD , const LIMBS : usize > Sum < & 'a Self > for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn sum < I : Iterator < Item = & 'a MontyFieldElement < MOD , LIMBS > > > (iter : I) -> Self { iter . copied () . sum () } }
};
}
