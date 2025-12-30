// Generated macro for impl_353 (impl)
macro_rules! Depcrate_connect_rustls_0_23impl_353 {
() => {
// Module: crate::connect::rustls_0_23
// Provides: {"impl_353"}
// Dependencies: {}
impl TlsConnector { # [doc = " Constructs new connector service factory from a `rustls` client configuration."] pub fn new (connector : Arc < ClientConfig >) -> Self { TlsConnector { connector } } # [doc = " Constructs new connector service from a `rustls` client configuration."] pub fn service (connector : Arc < ClientConfig >) -> TlsConnectorService { TlsConnectorService { connector } } }
};
}
