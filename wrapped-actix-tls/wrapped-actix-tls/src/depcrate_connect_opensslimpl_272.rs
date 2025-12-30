// Generated macro for impl_272 (impl)
macro_rules! Depcrate_connect_opensslimpl_272 {
() => {
// Module: crate::connect::openssl
// Provides: {"impl_272"}
// Dependencies: {}
impl TlsConnector { # [doc = " Constructs new connector service factory from an `openssl` connector."] pub fn new (connector : SslConnector) -> Self { TlsConnector { connector } } # [doc = " Constructs new connector service from an `openssl` connector."] pub fn service (connector : SslConnector) -> TlsConnectorService { TlsConnectorService { connector } } }
};
}
