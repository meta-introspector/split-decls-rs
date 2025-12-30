// Generated macro for impl_50 (impl)
macro_rules! Depcrate_commitimpl_50 {
() => {
// Module: crate::commit
// Provides: {"impl_50"}
// Dependencies: {}
impl SignedData < '_ > { # [doc = " Convenience method to obtain a copy of the signed data."] pub fn to_bstring (& self) -> BString { let mut buf = BString :: from (& self . data [.. self . signature_range . start]) ; buf . extend_from_slice (& self . data [self . signature_range . end ..]) ; buf } }
};
}
