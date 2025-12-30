// Generated macro for impl_124 (impl)
macro_rules! Depcrate_signingimpl_124 {
() => {
// Module: crate::signing
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "pkcs8"))] impl < C > EncodePrivateKey for SigningKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn to_pkcs8_der (& self) -> pkcs8 :: Result < SecretDocument > { SecretKey :: from (self . secret_scalar) . to_pkcs8_der () } }
};
}
