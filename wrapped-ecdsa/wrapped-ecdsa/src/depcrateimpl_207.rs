// Generated macro for impl_207 (impl)
macro_rules! Depcrateimpl_207 {
() => {
// Module: crate
// Provides: {"impl_207"}
// Dependencies: {}
impl < C > fmt :: LowerHex for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for byte in self . to_bytes () { write ! (f , "{byte:02x}") ? ; } Ok (()) } }
};
}
