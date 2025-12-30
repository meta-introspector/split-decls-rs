// Generated macro for impl_43 (impl)
macro_rules! Depcrate_ecdsaimpl_43 {
() => {
// Module: crate::ecdsa
// Provides: {"impl_43"}
// Dependencies: {}
impl Debug for Signature { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "bignp256::dsa::Signature(") ? ; for byte in self . to_bytes () { write ! (f , "{byte:02X}") ? ; } write ! (f , ")") } }
};
}
