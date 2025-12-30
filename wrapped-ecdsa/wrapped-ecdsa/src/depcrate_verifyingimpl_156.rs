// Generated macro for impl_156 (impl)
macro_rules! Depcrate_verifyingimpl_156 {
() => {
// Module: crate::verifying
// Provides: {"impl_156"}
// Dependencies: {}
impl < C > From < & VerifyingKey < C > > for EncodedPoint < C > where C : EcdsaCurve + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn from (verifying_key : & VerifyingKey < C >) -> EncodedPoint < C > { verifying_key . inner . into () } }
};
}
