// Generated macro for impl_72 (impl)
macro_rules! Depcrate_montyimpl_72 {
() => {
// Module: crate::monty
// Provides: {"impl_72"}
// Dependencies: {}
impl < MOD : MontyFieldParams < LIMBS > , const LIMBS : usize > PartialOrd for MontyFieldElement < MOD , LIMBS > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
};
}
