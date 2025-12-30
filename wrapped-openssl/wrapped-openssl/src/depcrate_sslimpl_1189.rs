// Generated macro for impl_1189 (impl)
macro_rules! Depcrate_sslimpl_1189 {
() => {
// Module: crate::ssl
// Provides: {"impl_1189"}
// Dependencies: {}
# [cfg (ossl111)] impl ClientHelloResponse { # [doc = " Continue the handshake."] pub const SUCCESS : ClientHelloResponse = ClientHelloResponse (ffi :: SSL_CLIENT_HELLO_SUCCESS) ; # [doc = " Return from the handshake with an `ErrorCode::WANT_CLIENT_HELLO_CB` error."] pub const RETRY : ClientHelloResponse = ClientHelloResponse (ffi :: SSL_CLIENT_HELLO_RETRY) ; }
};
}
