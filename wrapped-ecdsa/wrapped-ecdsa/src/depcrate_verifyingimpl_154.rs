// Generated macro for impl_154 (impl)
macro_rules! Depcrate_verifyingimpl_154 {
() => {
// Module: crate::verifying
// Provides: {"impl_154"}
// Dependencies: {}
impl < C > From < & VerifyingKey < C > > for CompressedPoint < C > where C : EcdsaCurve + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn from (verifying_key : & VerifyingKey < C >) -> CompressedPoint < C > { verifying_key . inner . into () } }
};
}
