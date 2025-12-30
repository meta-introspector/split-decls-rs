// Generated macro for TlsStream (struct)
macro_rules! Depcrate_accept_opensslTlsStream {
() => {
// Module: crate::accept::openssl
// Provides: {"TlsStream"}
// Dependencies: {}
# [doc = " Wraps an `openssl` based async TLS stream in order to implement [`ActixStream`]."] pub struct TlsStream < IO > (tokio_openssl :: SslStream < IO >) ;
};
}
