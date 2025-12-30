// Generated macro for impl_59 (impl)
macro_rules! Depcrate_montyimpl_59 {
() => {
// Module: crate::monty
// Provides: {"impl_59"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > fmt :: UpperHex for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (& self . to_canonical () , f) } }
};
}
