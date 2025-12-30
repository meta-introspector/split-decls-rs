// Generated macro for impl_884 (impl)
macro_rules! Depcrate_transport_errorimpl_884 {
() => {
// Module: crate::transport_error
// Provides: {"impl_884"}
// Dependencies: {}
impl Code { # [doc = " Create QUIC error code from TLS alert code"] pub fn crypto (code : u8) -> Self { Self (0x100 | u64 :: from (code)) } }
};
}
