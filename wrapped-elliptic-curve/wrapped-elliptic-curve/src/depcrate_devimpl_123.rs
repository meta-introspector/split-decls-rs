// Generated macro for impl_123 (impl)
macro_rules! Depcrate_devimpl_123 {
() => {
// Module: crate::dev
// Provides: {"impl_123"}
// Dependencies: {}
impl group :: GroupEncoding for AffinePoint { type Repr = CompressedPoint < MockCurve > ; fn from_bytes (bytes : & Self :: Repr) -> CtOption < Self > { EncodedPoint :: from_bytes (bytes) . map (| point | CtOption :: new (point , Choice :: from (1))) . unwrap_or_else (| _ | { let is_identity = bytes . ct_eq (& Self :: Repr :: default ()) ; CtOption :: new (EncodedPoint :: identity () , is_identity) }) . and_then (| point | Self :: from_encoded_point (& point)) } fn from_bytes_unchecked (bytes : & Self :: Repr) -> CtOption < Self > { Self :: from_bytes (bytes) } fn to_bytes (& self) -> Self :: Repr { let encoded = self . to_encoded_point (true) ; let mut result = CompressedPoint :: < MockCurve > :: default () ; result [.. encoded . len ()] . copy_from_slice (encoded . as_bytes ()) ; result } }
};
}
