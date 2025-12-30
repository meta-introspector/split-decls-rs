// Generated macro for SslConnector (struct)
macro_rules! Depcrate_ssl_connectorSslConnector {
() => {
// Module: crate::ssl::connector
// Provides: {"SslConnector"}
// Dependencies: {}
# [doc = " A type which wraps client-side streams in a TLS session."] # [doc = ""] # [doc = " OpenSSL's default configuration is highly insecure. This connector manages the OpenSSL"] # [doc = " structures, configuring cipher suites, session options, hostname verification, and more."] # [derive (Clone , Debug)] pub struct SslConnector (SslContext) ;
};
}
