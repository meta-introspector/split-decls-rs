// Generated macro for impl_90 (impl)
macro_rules! Depcrate_signingimpl_90 {
() => {
// Module: crate::signing
// Provides: {"impl_90"}
// Dependencies: {}
impl < C > MultipartSigner < Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_multipart_sign (& self , msg : & [& [u8]]) -> core :: result :: Result < Signature < C > , Error > { self . try_sign_digest (| digest : & mut C :: Digest | { msg . iter () . for_each (| slice | digest . update (slice)) ; Ok (()) }) } }
};
}
