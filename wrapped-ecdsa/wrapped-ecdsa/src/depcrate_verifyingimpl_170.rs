// Generated macro for impl_170 (impl)
macro_rules! Depcrate_verifyingimpl_170 {
() => {
// Module: crate::verifying
// Provides: {"impl_170"}
// Dependencies: {}
# [cfg (feature = "pem")] impl < C > FromStr for VerifyingKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { type Err = Error ; fn from_str (s : & str) -> Result < Self > { Self :: from_public_key_pem (s) . map_err (| _ | Error :: new ()) } }
};
}
