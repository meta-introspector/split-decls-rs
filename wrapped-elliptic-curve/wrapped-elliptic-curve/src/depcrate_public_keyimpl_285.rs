// Generated macro for impl_285 (impl)
macro_rules! Depcrate_public_keyimpl_285 {
() => {
// Module: crate::public_key
// Provides: {"impl_285"}
// Dependencies: {}
# [cfg (feature = "sec1")] impl < C > PartialOrd for PublicKey < C > where C : CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : ModulusSize , { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
};
}
