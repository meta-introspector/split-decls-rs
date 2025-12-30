// Generated macro for impl_155 (impl)
macro_rules! Depcrate_verifyingimpl_155 {
() => {
// Module: crate::verifying
// Provides: {"impl_155"}
// Dependencies: {}
impl < C > From < VerifyingKey < C > > for EncodedPoint < C > where C : EcdsaCurve + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn from (verifying_key : VerifyingKey < C >) -> EncodedPoint < C > { verifying_key . inner . into () } }
};
}
