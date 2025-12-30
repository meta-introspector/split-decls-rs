// Generated macro for impl_24 (impl)
macro_rules! Depcrate_ecdhimpl_24 {
() => {
// Module: crate::ecdh
// Provides: {"impl_24"}
// Dependencies: {}
impl From < & AffinePoint > for SharedSecret { fn from (affine : & AffinePoint) -> SharedSecret { affine . x . to_bytes () . into () } }
};
}
