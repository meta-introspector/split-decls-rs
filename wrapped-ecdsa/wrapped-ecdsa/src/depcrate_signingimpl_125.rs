// Generated macro for impl_125 (impl)
macro_rules! Depcrate_signingimpl_125 {
() => {
// Module: crate::signing
// Provides: {"impl_125"}
// Dependencies: {}
# [cfg (feature = "pem")] impl < C > FromStr for SigningKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { type Err = Error ; fn from_str (s : & str) -> Result < Self > { Self :: from_pkcs8_pem (s) . map_err (| _ | Error :: new ()) } }
};
}
