// Generated macro for impl_165 (impl)
macro_rules! Depcrate_verifyingimpl_165 {
() => {
// Module: crate::verifying
// Provides: {"impl_165"}
// Dependencies: {}
impl < C > TryFrom < & [u8] > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { type Error = Error ; fn try_from (bytes : & [u8]) -> Result < Self > { Self :: from_sec1_bytes (bytes) } }
};
}
