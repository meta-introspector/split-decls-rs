// Generated macro for impl_123 (impl)
macro_rules! Depcrate_accept_rustls_0_23impl_123 {
() => {
// Module: crate::accept::rustls_0_23
// Provides: {"impl_123"}
// Dependencies: {}
impl Acceptor { # [doc = " Constructs `rustls` based acceptor service factory."] pub fn new (config : reexports :: ServerConfig) -> Self { Acceptor { config : Arc :: new (config) , handshake_timeout : DEFAULT_TLS_HANDSHAKE_TIMEOUT , } } # [doc = " Limit the amount of time that the acceptor will wait for a TLS handshake to complete."] # [doc = ""] # [doc = " Default timeout is 3 seconds."] pub fn set_handshake_timeout (& mut self , handshake_timeout : Duration) -> & mut Self { self . handshake_timeout = handshake_timeout ; self } }
};
}
