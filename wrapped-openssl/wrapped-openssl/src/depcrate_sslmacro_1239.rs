// Generated macro for macro_1239 (macro)
macro_rules! Depcrate_sslmacro_1239 {
() => {
// Module: crate::ssl
// Provides: {"macro_1239"}
// Dependencies: {}
cfg_if ! { if # [cfg (ossl300)] { use ffi :: SSL_get1_peer_certificate ; } else { use ffi :: SSL_get_peer_certificate as SSL_get1_peer_certificate ; } }
};
}
