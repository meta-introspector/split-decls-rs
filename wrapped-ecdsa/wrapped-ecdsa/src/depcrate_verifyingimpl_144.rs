// Generated macro for impl_144 (impl)
macro_rules! Depcrate_verifyingimpl_144 {
() => {
// Module: crate::verifying
// Provides: {"impl_144"}
// Dependencies: {}
impl < C > MultipartVerifier < Signature < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , SignatureSize < C > : ArraySize , { fn multipart_verify (& self , msg : & [& [u8]] , signature : & Signature < C >) -> Result < () > { self . verify_digest (| digest : & mut C :: Digest | { msg . iter () . for_each (| slice | digest . update (slice)) ; Ok (()) } , signature ,) } }
};
}
