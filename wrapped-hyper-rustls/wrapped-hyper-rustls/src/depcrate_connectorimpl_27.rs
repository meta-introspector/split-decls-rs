// Generated macro for impl_27 (impl)
macro_rules! Depcrate_connectorimpl_27 {
() => {
// Module: crate::connector
// Provides: {"impl_27"}
// Dependencies: {}
impl < T > HttpsConnector < T > { # [doc = " Creates a [`crate::HttpsConnectorBuilder`] to configure a `HttpsConnector`."] # [doc = ""] # [doc = " This is the same as [`crate::HttpsConnectorBuilder::new()`]."] pub fn builder () -> builder :: ConnectorBuilder < builder :: WantsTlsConfig > { builder :: ConnectorBuilder :: new () } # [doc = " Creates a new `HttpsConnector`."] # [doc = ""] # [doc = " The recommended way to create a `HttpsConnector` is to use a [`crate::HttpsConnectorBuilder`]. See [`HttpsConnector::builder()`]."] pub fn new (http : T , tls_config : impl Into < Arc < rustls :: ClientConfig > > , force_https : bool , server_name_resolver : Arc < dyn ResolveServerName + Send + Sync > ,) -> Self { Self { http , tls_config : tls_config . into () , force_https , server_name_resolver , } } # [doc = " Force the use of HTTPS when connecting."] # [doc = ""] # [doc = " If a URL is not `https` when connecting, an error is returned."] pub fn enforce_https (& mut self) { self . force_https = true ; } }
};
}
