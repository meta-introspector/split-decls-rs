// Generated macro for macro_1164 (macro)
macro_rules! Depcrate_sslmacro_1164 {
() => {
// Module: crate::ssl
// Provides: {"macro_1164"}
// Dependencies: {}
bitflags ! { # [doc = " Options controlling the behavior of certificate verification."] # [derive (Copy , Clone , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct SslVerifyMode : i32 { # [doc = " Verifies that the peer's certificate is trusted."] # [doc = ""] # [doc = " On the server side, this will cause OpenSSL to request a certificate from the client."] const PEER = ffi :: SSL_VERIFY_PEER ; # [doc = " Disables verification of the peer's certificate."] # [doc = ""] # [doc = " On the server side, this will cause OpenSSL to not request a certificate from the"] # [doc = " client. On the client side, the certificate will be checked for validity, but the"] # [doc = " negotiation will continue regardless of the result of that check."] const NONE = ffi :: SSL_VERIFY_NONE ; # [doc = " On the server side, abort the handshake if the client did not send a certificate."] # [doc = ""] # [doc = " This should be paired with `SSL_VERIFY_PEER`. It has no effect on the client side."] const FAIL_IF_NO_PEER_CERT = ffi :: SSL_VERIFY_FAIL_IF_NO_PEER_CERT ; } }
};
}
