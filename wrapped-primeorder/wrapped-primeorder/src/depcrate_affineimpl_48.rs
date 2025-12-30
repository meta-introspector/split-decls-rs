// Generated macro for impl_48 (impl)
macro_rules! Depcrate_affineimpl_48 {
() => {
// Module: crate::affine
// Provides: {"impl_48"}
// Dependencies: {}
impl < C > ToCompactEncodedPoint < C > for AffinePoint < C > where C : PrimeCurveParams , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { # [doc = " Serialize this value as a  SEC1 compact [`EncodedPoint`]"] fn to_compact_encoded_point (& self) -> CtOption < EncodedPoint < C > > { let point = self . to_compact () ; let mut bytes = CompressedPoint :: < C > :: default () ; bytes [0] = sec1 :: Tag :: Compact . into () ; bytes [1 ..] . copy_from_slice (& point . x . to_repr ()) ; let encoded = EncodedPoint :: < C > :: from_bytes (bytes) ; let is_some = point . y . ct_eq (& self . y) ; CtOption :: new (encoded . unwrap_or_default () , is_some) } }
};
}
