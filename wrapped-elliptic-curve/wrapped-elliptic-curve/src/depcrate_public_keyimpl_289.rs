// Generated macro for impl_289 (impl)
macro_rules! Depcrate_public_keyimpl_289 {
() => {
// Module: crate::public_key
// Provides: {"impl_289"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > TryFrom < EncodedPoint < C > > for PublicKey < C > where C : CurveArithmetic , FieldBytesSize < C > : ModulusSize , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , { type Error = Error ; fn try_from (point : EncodedPoint < C >) -> Result < Self > { Self :: from_sec1_bytes (point . as_bytes ()) } }
};
}
