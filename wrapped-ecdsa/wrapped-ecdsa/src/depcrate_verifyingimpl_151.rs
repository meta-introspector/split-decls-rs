// Generated macro for impl_151 (impl)
macro_rules! Depcrate_verifyingimpl_151 {
() => {
// Module: crate::verifying
// Provides: {"impl_151"}
// Dependencies: {}
impl < C > AsRef < AffinePoint < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn as_ref (& self) -> & AffinePoint < C > { self . as_affine () } }
};
}
