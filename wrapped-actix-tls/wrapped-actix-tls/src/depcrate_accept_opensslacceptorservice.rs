// Generated macro for AcceptorService (struct)
macro_rules! Depcrate_accept_opensslAcceptorService {
() => {
// Module: crate::accept::openssl
// Provides: {"AcceptorService"}
// Dependencies: {}
# [doc = " OpenSSL based acceptor service."] pub struct AcceptorService { acceptor : SslAcceptor , conns : Counter , handshake_timeout : Duration , }
};
}
