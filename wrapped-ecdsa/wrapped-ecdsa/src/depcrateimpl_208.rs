// Generated macro for impl_208 (impl)
macro_rules! Depcrateimpl_208 {
() => {
// Module: crate
// Provides: {"impl_208"}
// Dependencies: {}
impl < C > fmt :: UpperHex for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for byte in self . to_bytes () { write ! (f , "{byte:02X}") ? ; } Ok (()) } }
};
}
