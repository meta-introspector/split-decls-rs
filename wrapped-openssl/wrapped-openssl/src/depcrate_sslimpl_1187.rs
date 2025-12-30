// Generated macro for impl_1187 (impl)
macro_rules! Depcrate_sslimpl_1187 {
() => {
// Module: crate::ssl
// Provides: {"impl_1187"}
// Dependencies: {}
impl AlpnError { # [doc = " Terminate the handshake with a fatal alert."] # [doc = ""] # [doc = " Requires AWS-LC or BoringSSL or OpenSSL 1.1.0 or newer."] # [cfg (any (ossl110 , libressl , boringssl , awslc))] pub const ALERT_FATAL : AlpnError = AlpnError (ffi :: SSL_TLSEXT_ERR_ALERT_FATAL) ; # [doc = " Do not select a protocol, but continue the handshake."] pub const NOACK : AlpnError = AlpnError (ffi :: SSL_TLSEXT_ERR_NOACK) ; }
};
}
