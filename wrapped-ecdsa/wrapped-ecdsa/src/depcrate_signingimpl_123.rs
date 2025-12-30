// Generated macro for impl_123 (impl)
macro_rules! Depcrate_signingimpl_123 {
() => {
// Module: crate::signing
// Provides: {"impl_123"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl < C > TryFrom < pkcs8 :: PrivateKeyInfoRef < '_ > > for SigningKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { type Error = pkcs8 :: Error ; fn try_from (private_key_info : pkcs8 :: PrivateKeyInfoRef < '_ >) -> pkcs8 :: Result < Self > { SecretKey :: try_from (private_key_info) . map (Into :: into) } }
};
}
