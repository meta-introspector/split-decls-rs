// Generated macro for impl_57 (impl)
macro_rules! Depcrate_montyimpl_57 {
() => {
// Module: crate::monty
// Provides: {"impl_57"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > fmt :: Binary for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Binary :: fmt (& self . to_canonical () , f) } }
};
}
