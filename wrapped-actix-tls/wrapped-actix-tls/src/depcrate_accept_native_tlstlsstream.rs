// Generated macro for TlsStream (struct)
macro_rules! Depcrate_accept_native_tlsTlsStream {
() => {
// Module: crate::accept::native_tls
// Provides: {"TlsStream"}
// Dependencies: {}
# [doc = " Wraps a `native-tls` based async TLS stream in order to implement [`ActixStream`]."] pub struct TlsStream < IO > (tokio_native_tls :: TlsStream < IO >) ;
};
}
