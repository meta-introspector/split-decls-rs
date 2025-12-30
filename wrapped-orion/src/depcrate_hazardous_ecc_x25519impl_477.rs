// Generated macro for impl_477 (impl)
macro_rules! Depcrate_hazardous_ecc_x25519impl_477 {
() => {
// Module: crate::hazardous::ecc::x25519
// Provides: {"impl_477"}
// Dependencies: {}
impl From < [u8 ; PUBLIC_KEY_SIZE] > for PublicKey { # [inline] fn from (bytes : [u8 ; PUBLIC_KEY_SIZE]) -> Self { Self { fe : FieldElement :: from_bytes (& bytes) , } } }
};
}
