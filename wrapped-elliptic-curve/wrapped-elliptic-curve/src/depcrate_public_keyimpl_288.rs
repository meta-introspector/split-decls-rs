// Generated macro for impl_288 (impl)
macro_rules! Depcrate_public_keyimpl_288 {
() => {
// Module: crate::public_key
// Provides: {"impl_288"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > TryFrom < & CompressedPoint < C > > for PublicKey < C > where C : CurveArithmetic , FieldBytesSize < C > : ModulusSize , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , { type Error = Error ; fn try_from (point : & CompressedPoint < C >) -> Result < Self > { Self :: from_sec1_bytes (point) } }
};
}
