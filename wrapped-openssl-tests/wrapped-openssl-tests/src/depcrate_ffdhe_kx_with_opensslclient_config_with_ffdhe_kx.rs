// Generated macro for client_config_with_ffdhe_kx (function)
macro_rules! Depcrate_ffdhe_kx_with_opensslclient_config_with_ffdhe_kx {
() => {
// Module: crate::ffdhe_kx_with_openssl
// Provides: {"client_config_with_ffdhe_kx"}
// Dependencies: {}
fn client_config_with_ffdhe_kx () -> ClientConfig { ClientConfig :: builder (FFDHE_TLS13_PROVIDER . into () ,) . with_root_certificates (root_ca ()) . with_no_client_auth () . unwrap () }
};
}
