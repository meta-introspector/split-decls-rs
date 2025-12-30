// Generated macro for server_crypto_with_cert (function)
macro_rules! Depcrate_tests_utilserver_crypto_with_cert {
() => {
// Module: crate::tests::util
// Provides: {"server_crypto_with_cert"}
// Dependencies: {}
pub (super) fn server_crypto_with_cert (cert : CertificateDer < 'static > , key : PrivateKeyDer < 'static > ,) -> QuicServerConfig { server_crypto_inner (Some ((cert , key)) , None) }
};
}
