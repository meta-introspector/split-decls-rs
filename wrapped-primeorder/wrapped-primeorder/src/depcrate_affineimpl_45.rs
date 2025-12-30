// Generated macro for impl_45 (impl)
macro_rules! Depcrate_affineimpl_45 {
() => {
// Module: crate::affine
// Provides: {"impl_45"}
// Dependencies: {}
impl < C > GroupEncoding for AffinePoint < C > where C : PrimeCurveParams , CompressedPoint < C > : Copy + Send + Sync , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { type Repr = CompressedPoint < C > ; # [doc = " NOTE: not constant-time with respect to identity point"] fn from_bytes (bytes : & Self :: Repr) -> CtOption < Self > { EncodedPoint :: < C > :: from_bytes (bytes) . map (| point | CtOption :: new (point , Choice :: from (1))) . unwrap_or_else (| _ | { let is_identity = bytes . ct_eq (& Self :: Repr :: default ()) ; CtOption :: new (EncodedPoint :: < C > :: identity () , is_identity) }) . and_then (| point | Self :: from_encoded_point (& point)) } fn from_bytes_unchecked (bytes : & Self :: Repr) -> CtOption < Self > { Self :: from_bytes (bytes) } fn to_bytes (& self) -> Self :: Repr { let encoded = self . to_encoded_point (true) ; let mut result = CompressedPoint :: < C > :: default () ; result [.. encoded . len ()] . copy_from_slice (encoded . as_bytes ()) ; result } }
};
}
