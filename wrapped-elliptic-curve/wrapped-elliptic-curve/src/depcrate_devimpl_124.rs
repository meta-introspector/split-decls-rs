// Generated macro for impl_124 (impl)
macro_rules! Depcrate_devimpl_124 {
() => {
// Module: crate::dev
// Provides: {"impl_124"}
// Dependencies: {}
impl group :: GroupEncoding for ProjectivePoint { type Repr = CompressedPoint < MockCurve > ; fn from_bytes (bytes : & Self :: Repr) -> CtOption < Self > { < AffinePoint as group :: GroupEncoding > :: from_bytes (bytes) . map (Into :: into) } fn from_bytes_unchecked (bytes : & Self :: Repr) -> CtOption < Self > { Self :: from_bytes (bytes) } fn to_bytes (& self) -> Self :: Repr { CurveGroup :: to_affine (self) . to_bytes () } }
};
}
