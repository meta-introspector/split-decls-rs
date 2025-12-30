// Generated macro for impl_168 (impl)
macro_rules! Depcrate_verifyingimpl_168 {
() => {
// Module: crate::verifying
// Provides: {"impl_168"}
// Dependencies: {}
# [cfg (feature = "pkcs8")] impl < C > TryFrom < pkcs8 :: SubjectPublicKeyInfoRef < '_ > > for VerifyingKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { type Error = spki :: Error ; fn try_from (spki : pkcs8 :: SubjectPublicKeyInfoRef < '_ >) -> spki :: Result < Self > { PublicKey :: try_from (spki) . map (| inner | Self { inner }) } }
};
}
