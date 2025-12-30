// Generated macro for impl_145 (impl)
macro_rules! Depcrate_verifyingimpl_145 {
() => {
// Module: crate::verifying
// Provides: {"impl_145"}
// Dependencies: {}
# [cfg (feature = "sha2")] impl < C > Verifier < SignatureWithOid < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , SignatureSize < C > : ArraySize , { fn verify (& self , msg : & [u8] , sig : & SignatureWithOid < C >) -> Result < () > { self . multipart_verify (& [msg] , sig) } }
};
}
