// Generated macro for impl_205 (impl)
macro_rules! Depcrateimpl_205 {
() => {
// Module: crate
// Provides: {"impl_205"}
// Dependencies: {}
impl < C > fmt :: Debug for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ecdsa::Signature<{:?}>(" , C :: default ()) ? ; for byte in self . to_bytes () { write ! (f , "{byte:02X}") ? ; } write ! (f , ")") } }
};
}
