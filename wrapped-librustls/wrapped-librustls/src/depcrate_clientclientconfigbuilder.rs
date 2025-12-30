// Generated macro for ClientConfigBuilder (struct)
macro_rules! Depcrate_clientClientConfigBuilder {
() => {
// Module: crate::client
// Provides: {"ClientConfigBuilder"}
// Dependencies: {}
pub (crate) struct ClientConfigBuilder { provider : Option < Arc < CryptoProvider > > , versions : Vec < & 'static SupportedProtocolVersion > , verifier : Option < Arc < dyn ServerCertVerifier > > , alpn_protocols : Vec < Vec < u8 > > , enable_sni : bool , cert_resolver : Option < Arc < dyn ResolvesClientCert > > , key_log : Option < Arc < dyn KeyLog > > , ech_mode : Option < EchMode > , }
};
}
