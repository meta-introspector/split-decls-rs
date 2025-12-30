// Generated macro for load_private_key (function)
macro_rules! Depcrate_ffdhe_kx_with_opensslload_private_key {
() => {
// Module: crate::ffdhe_kx_with_openssl
// Provides: {"load_private_key"}
// Dependencies: {}
fn load_private_key () -> PrivateKeyDer < 'static > { PrivateKeyDer :: from_pem_file (PRIV_KEY_FILE) . unwrap () }
};
}
