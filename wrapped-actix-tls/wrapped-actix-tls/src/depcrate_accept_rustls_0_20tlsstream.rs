// Generated macro for TlsStream (struct)
macro_rules! Depcrate_accept_rustls_0_20TlsStream {
() => {
// Module: crate::accept::rustls_0_20
// Provides: {"TlsStream"}
// Dependencies: {}
# [doc = " Wraps a `rustls` based async TLS stream in order to implement [`ActixStream`]."] pub struct TlsStream < IO > (tokio_rustls :: server :: TlsStream < IO >) ;
};
}
