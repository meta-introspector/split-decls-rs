// Generated macro for HandshakeError (enum)
macro_rules! Depcrate_ssl_errorHandshakeError {
() => {
// Module: crate::ssl::error
// Provides: {"HandshakeError"}
// Dependencies: {}
# [doc = " An error or intermediate state after a TLS handshake attempt."] # [derive (Debug)] pub enum HandshakeError < S > { # [doc = " Setup failed."] SetupFailure (ErrorStack) , # [doc = " The handshake failed."] Failure (MidHandshakeSslStream < S >) , # [doc = " The handshake encountered a `WouldBlock` error midway through."] # [doc = ""] # [doc = " This error will never be returned for blocking streams."] WouldBlock (MidHandshakeSslStream < S >) , }
};
}
