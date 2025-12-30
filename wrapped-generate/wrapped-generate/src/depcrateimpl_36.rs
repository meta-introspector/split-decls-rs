// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl fmt :: Display for UIntCode { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { UIntCode :: Term => write ! (f , "UTerm") , UIntCode :: Zero (ref inner) => write ! (f , "UInt<{}, B0>" , inner) , UIntCode :: One (ref inner) => write ! (f , "UInt<{}, B1>" , inner) , } } }
};
}
