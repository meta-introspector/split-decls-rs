// Generated macro for impl_143 (impl)
macro_rules! Depcrate_verifyingimpl_143 {
() => {
// Module: crate::verifying
// Provides: {"impl_143"}
// Dependencies: {}
impl < C > Verifier < Signature < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , SignatureSize < C > : ArraySize , { fn verify (& self , msg : & [u8] , signature : & Signature < C >) -> Result < () > { self . multipart_verify (& [msg] , signature) } }
};
}
