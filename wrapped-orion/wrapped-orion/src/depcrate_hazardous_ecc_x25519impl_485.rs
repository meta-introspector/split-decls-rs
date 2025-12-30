// Generated macro for impl_485 (impl)
macro_rules! Depcrate_hazardous_ecc_x25519impl_485 {
() => {
// Module: crate::hazardous::ecc::x25519
// Provides: {"impl_485"}
// Dependencies: {}
impl From < [u8 ; PRIVATE_KEY_SIZE] > for PrivateKey { # [inline] fn from (bytes : [u8 ; PRIVATE_KEY_SIZE]) -> Self { PrivateKey { scalar : Scalar :: from_slice (bytes . as_ref ()) . unwrap () , } } }
};
}
