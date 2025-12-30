// Generated macro for impl_313 (impl)
macro_rules! Depcrate_connect_rustls_0_21impl_313 {
() => {
// Module: crate::connect::rustls_0_21
// Provides: {"impl_313"}
// Dependencies: {}
impl TlsConnector { # [doc = " Constructs new connector service factory from a `rustls` client configuration."] pub fn new (connector : Arc < ClientConfig >) -> Self { TlsConnector { connector } } # [doc = " Constructs new connector service from a `rustls` client configuration."] pub fn service (connector : Arc < ClientConfig >) -> TlsConnectorService { TlsConnectorService { connector } } }
};
}
