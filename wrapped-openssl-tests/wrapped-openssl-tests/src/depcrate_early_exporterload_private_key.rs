// Generated macro for load_private_key (function)
macro_rules! Depcrate_early_exporterload_private_key {
() => {
// Module: crate::early_exporter
// Provides: {"load_private_key"}
// Dependencies: {}
fn load_private_key () -> PrivateKeyDer < 'static > { PrivateKeyDer :: from_pem_file (PRIV_KEY_FILE) . unwrap () }
};
}
