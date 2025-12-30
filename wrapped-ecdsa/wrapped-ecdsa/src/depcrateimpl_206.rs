// Generated macro for impl_206 (impl)
macro_rules! Depcrateimpl_206 {
() => {
// Module: crate
// Provides: {"impl_206"}
// Dependencies: {}
impl < C > fmt :: Display for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{self:X}") } }
};
}
