// Generated macro for impl_96 (impl)
macro_rules! Depcrate_signingimpl_96 {
() => {
// Module: crate::signing
// Provides: {"impl_96"}
// Dependencies: {}
impl < C > Signer < SignatureWithOid < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , C :: Digest : AssociatedOid , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_sign (& self , msg : & [u8]) -> Result < SignatureWithOid < C > > { self . try_multipart_sign (& [msg]) } }
};
}
