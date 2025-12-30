// Generated macro for impl_111 (impl)
macro_rules! Depcrate_agreementimpl_111 {
() => {
// Module: crate::agreement
// Provides: {"impl_111"}
// Dependencies: {}
impl AsRef < [u8] > for PublicKey { # [doc = " Serializes the public key in an uncompressed form (X9.62) using the"] # [doc = " Octet-String-to-Elliptic-Curve-Point algorithm in"] # [doc = " [SEC 1: Elliptic Curve Cryptography, Version 2.0]."] fn as_ref (& self) -> & [u8] { & self . key_bytes [0 .. self . len] } }
};
}
