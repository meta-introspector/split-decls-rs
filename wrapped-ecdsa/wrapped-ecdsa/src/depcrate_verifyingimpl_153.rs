// Generated macro for impl_153 (impl)
macro_rules! Depcrate_verifyingimpl_153 {
() => {
// Module: crate::verifying
// Provides: {"impl_153"}
// Dependencies: {}
impl < C > From < VerifyingKey < C > > for CompressedPoint < C > where C : EcdsaCurve + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn from (verifying_key : VerifyingKey < C >) -> CompressedPoint < C > { verifying_key . inner . into () } }
};
}
