// Generated macro for impl_290 (impl)
macro_rules! Depcrate_public_keyimpl_290 {
() => {
// Module: crate::public_key
// Provides: {"impl_290"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > TryFrom < & EncodedPoint < C > > for PublicKey < C > where C : CurveArithmetic , FieldBytesSize < C > : ModulusSize , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , { type Error = Error ; fn try_from (point : & EncodedPoint < C >) -> Result < Self > { Self :: from_sec1_bytes (point . as_bytes ()) } }
};
}
