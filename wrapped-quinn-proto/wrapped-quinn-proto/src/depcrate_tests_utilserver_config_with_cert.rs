// Generated macro for server_config_with_cert (function)
macro_rules! Depcrate_tests_utilserver_config_with_cert {
() => {
// Module: crate::tests::util
// Provides: {"server_config_with_cert"}
// Dependencies: {}
pub (super) fn server_config_with_cert (cert : CertificateDer < 'static > , key : PrivateKeyDer < 'static > ,) -> ServerConfig { let mut config = ServerConfig :: with_crypto (Arc :: new (server_crypto_with_cert (cert , key))) ; config . validation_token . sent (2) . log (Arc :: new (SimpleTokenLog :: default ())) ; config }
};
}
