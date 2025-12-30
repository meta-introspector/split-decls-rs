// Generated macro for impl_483 (impl)
macro_rules! Depcrate_hazardous_ecc_x25519impl_483 {
() => {
// Module: crate::hazardous::ecc::x25519
// Provides: {"impl_483"}
// Dependencies: {}
impl PartialEq < & [u8] > for PrivateKey { fn eq (& self , other : & & [u8]) -> bool { match Scalar :: from_slice (other) { Ok (other_scalar) => self . scalar == other_scalar , Err (_) => false , } } }
};
}
