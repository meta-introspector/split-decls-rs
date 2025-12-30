// Generated macro for impl_58 (impl)
macro_rules! Depcrate_montyimpl_58 {
() => {
// Module: crate::monty
// Provides: {"impl_58"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > fmt :: LowerHex for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . to_canonical () , f) } }
};
}
