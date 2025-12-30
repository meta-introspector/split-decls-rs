// Generated macro for AcceptorService (struct)
macro_rules! Depcrate_accept_native_tlsAcceptorService {
() => {
// Module: crate::accept::native_tls
// Provides: {"AcceptorService"}
// Dependencies: {}
# [doc = " Native-TLS based acceptor service."] pub struct AcceptorService { acceptor : TlsAcceptor , conns : Counter , handshake_timeout : Duration , }
};
}
