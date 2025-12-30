// Generated macro for impl_22 (impl)
macro_rules! Depcrate_accept_opensslimpl_22 {
() => {
// Module: crate::accept::openssl
// Provides: {"impl_22"}
// Dependencies: {}
impl Acceptor { # [doc = " Create `openssl` based acceptor service factory."] # [inline] pub fn new (acceptor : SslAcceptor) -> Self { Acceptor { acceptor , handshake_timeout : DEFAULT_TLS_HANDSHAKE_TIMEOUT , } } # [doc = " Limit the amount of time that the acceptor will wait for a TLS handshake to complete."] # [doc = ""] # [doc = " Default timeout is 3 seconds."] pub fn set_handshake_timeout (& mut self , handshake_timeout : Duration) -> & mut Self { self . handshake_timeout = handshake_timeout ; self } }
};
}
