// Generated macro for client_config_with_certs (function)
macro_rules! Depcrate_tests_utilclient_config_with_certs {
() => {
// Module: crate::tests::util
// Provides: {"client_config_with_certs"}
// Dependencies: {}
pub (super) fn client_config_with_certs (certs : Vec < CertificateDer < 'static > >) -> ClientConfig { ClientConfig :: new (Arc :: new (client_crypto_inner (Some (certs) , None))) }
};
}
