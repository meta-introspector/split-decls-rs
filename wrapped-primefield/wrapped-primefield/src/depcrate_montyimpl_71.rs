// Generated macro for impl_71 (impl)
macro_rules! Depcrate_montyimpl_71 {
() => {
// Module: crate::monty
// Provides: {"impl_71"}
// Dependencies: {}
impl < MOD : MontyFieldParams < LIMBS > , const LIMBS : usize > Ord for MontyFieldElement < MOD , LIMBS > { fn cmp (& self , other : & Self) -> Ordering { self . to_canonical () . cmp (& other . to_canonical ()) } }
};
}
