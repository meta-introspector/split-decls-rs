// Generated macro for impl_55 (impl)
macro_rules! Depcrate_montyimpl_55 {
() => {
// Module: crate::monty
// Provides: {"impl_55"}
// Dependencies: {}
impl < MOD , const LIMBS : usize > fmt :: Debug for MontyFieldElement < MOD , LIMBS > where MOD : MontyFieldParams < LIMBS > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let canonical = self . to_canonical () ; write ! (f , "MontyFieldElement<p={}>(0x{:X})" , MOD :: MODULUS_HEX , canonical) } }
};
}
