// Generated macro for impl_287 (impl)
macro_rules! Depcrate_public_keyimpl_287 {
() => {
// Module: crate::public_key
// Provides: {"impl_287"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > TryFrom < CompressedPoint < C > > for PublicKey < C > where C : CurveArithmetic , FieldBytesSize < C > : ModulusSize , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , { type Error = Error ; fn try_from (point : CompressedPoint < C >) -> Result < Self > { Self :: from_sec1_bytes (& point) } }
};
}
