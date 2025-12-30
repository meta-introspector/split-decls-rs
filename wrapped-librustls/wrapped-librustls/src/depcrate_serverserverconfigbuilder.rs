// Generated macro for ServerConfigBuilder (struct)
macro_rules! Depcrate_serverServerConfigBuilder {
() => {
// Module: crate::server
// Provides: {"ServerConfigBuilder"}
// Dependencies: {}
pub (crate) struct ServerConfigBuilder { provider : Option < Arc < CryptoProvider > > , versions : Vec < & 'static SupportedProtocolVersion > , verifier : Arc < dyn ClientCertVerifier > , cert_resolver : Option < Arc < dyn ResolvesServerCert > > , session_storage : Option < Arc < dyn StoresServerSessions + Send + Sync > > , alpn_protocols : Vec < Vec < u8 > > , ignore_client_order : Option < bool > , key_log : Option < Arc < dyn KeyLog > > , }
};
}
