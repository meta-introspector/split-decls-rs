// Generated macro for impl_169 (impl)
macro_rules! Depcrate_verifyingimpl_169 {
() => {
// Module: crate::verifying
// Provides: {"impl_169"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "pkcs8"))] impl < C > EncodePublicKey for VerifyingKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn to_public_key_der (& self) -> spki :: Result < pkcs8 :: Document > { self . inner . to_public_key_der () } }
};
}
