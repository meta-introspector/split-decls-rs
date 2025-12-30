// Generated macro for impl_1183 (impl)
macro_rules! Depcrate_sslimpl_1183 {
() => {
// Module: crate::ssl
// Provides: {"impl_1183"}
// Dependencies: {}
impl SniError { # [doc = " Abort the handshake with a fatal alert."] pub const ALERT_FATAL : SniError = SniError (ffi :: SSL_TLSEXT_ERR_ALERT_FATAL) ; # [doc = " Send a warning alert to the client and continue the handshake."] pub const ALERT_WARNING : SniError = SniError (ffi :: SSL_TLSEXT_ERR_ALERT_WARNING) ; pub const NOACK : SniError = SniError (ffi :: SSL_TLSEXT_ERR_NOACK) ; }
};
}
