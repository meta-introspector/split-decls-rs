// Generated macro for server_config_with_ffdhe_kx (function)
macro_rules! Depcrate_ffdhe_kx_with_opensslserver_config_with_ffdhe_kx {
() => {
// Module: crate::ffdhe_kx_with_openssl
// Provides: {"server_config_with_ffdhe_kx"}
// Dependencies: {}
fn server_config_with_ffdhe_kx (provider : CryptoProvider) -> ServerConfig { ServerConfig :: builder (provider . into ()) . with_no_client_auth () . with_single_cert (Arc :: new (Identity :: from_cert_chain (load_certs ()) . unwrap ()) , load_private_key () ,) . unwrap () }
};
}
