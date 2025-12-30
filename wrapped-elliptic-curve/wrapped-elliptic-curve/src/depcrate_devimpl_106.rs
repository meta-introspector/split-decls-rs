// Generated macro for impl_106 (impl)
macro_rules! Depcrate_devimpl_106 {
() => {
// Module: crate::dev
// Provides: {"impl_106"}
// Dependencies: {}
impl FromEncodedPoint < MockCurve > for AffinePoint { fn from_encoded_point (encoded_point : & EncodedPoint) -> CtOption < Self > { let point = if encoded_point . is_identity () { Self :: Identity } else { Self :: Other (* encoded_point) } ; CtOption :: new (point , Choice :: from (1)) } }
};
}
