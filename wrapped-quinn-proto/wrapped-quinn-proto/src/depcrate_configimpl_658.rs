// Generated macro for impl_658 (impl)
macro_rules! Depcrate_configimpl_658 {
() => {
// Module: crate::config
// Provides: {"impl_658"}
// Dependencies: {}
# [cfg (any (feature = "rustls-aws-lc-rs" , feature = "rustls-ring"))] impl ServerConfig { # [doc = " Create a server config with the given certificate chain to be presented to clients"] # [doc = ""] # [doc = " Uses a randomized handshake token key."] pub fn with_single_cert (cert_chain : Vec < CertificateDer < 'static > > , key : PrivateKeyDer < 'static > ,) -> Result < Self , rustls :: Error > { Ok (Self :: with_crypto (Arc :: new (QuicServerConfig :: new (cert_chain , key ,) ?))) } }
};
}
