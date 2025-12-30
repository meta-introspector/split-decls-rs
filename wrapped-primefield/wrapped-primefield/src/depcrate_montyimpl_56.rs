// Generated macro for impl_56 (impl)
macro_rules! Depcrate_montyimpl_56 {
() => {
// Module: crate::monty
// Provides: {"impl_56"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > fmt :: Display for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (self , f) } }
};
}
