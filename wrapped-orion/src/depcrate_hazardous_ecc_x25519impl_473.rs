// Generated macro for impl_473 (impl)
macro_rules! Depcrate_hazardous_ecc_x25519impl_473 {
() => {
// Module: crate::hazardous::ecc::x25519
// Provides: {"impl_473"}
// Dependencies: {}
impl Scalar { # [doc = " Create a scalar from some byte-array."] # [doc = " The scalar is clamped according to the RFC."] # [doc = ""] # [doc = " Ref: https://www.ietf.org/rfc/rfc7748.html#section-5"] fn from_slice (slice : & [u8]) -> Result < Scalar , UnknownCryptoError > { if slice . len () != PRIVATE_KEY_SIZE { return Err (UnknownCryptoError) ; } let mut ret = [0u8 ; PRIVATE_KEY_SIZE] ; ret . copy_from_slice (slice) ; ret [0] &= 248 ; ret [31] &= 127 ; ret [31] |= 64 ; Ok (Self (ret)) } }
};
}
