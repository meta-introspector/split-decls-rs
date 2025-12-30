// Generated macro for impl_147 (impl)
macro_rules! Depcrate_accept_native_tlsimpl_147 {
() => {
// Module: crate::accept::native_tls
// Provides: {"impl_147"}
// Dependencies: {}
impl Acceptor { # [doc = " Constructs `native-tls` based acceptor service factory."] pub fn new (acceptor : TlsAcceptor) -> Self { Acceptor { acceptor , handshake_timeout : DEFAULT_TLS_HANDSHAKE_TIMEOUT , } } # [doc = " Limit the amount of time that the acceptor will wait for a TLS handshake to complete."] # [doc = ""] # [doc = " Default timeout is 3 seconds."] pub fn set_handshake_timeout (& mut self , handshake_timeout : Duration) -> & mut Self { self . handshake_timeout = handshake_timeout ; self } }
};
}
