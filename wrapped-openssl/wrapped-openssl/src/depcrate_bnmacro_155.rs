// Generated macro for macro_155 (macro)
macro_rules! Depcrate_bnmacro_155 {
() => {
// Module: crate::bn
// Provides: {"macro_155"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , libressl))] { use ffi :: { BN_get_rfc2409_prime_1024 , BN_get_rfc2409_prime_768 } ; } else if # [cfg (not (any (boringssl , awslc)))] { use ffi :: { get_rfc2409_prime_1024 as BN_get_rfc2409_prime_1024 , get_rfc2409_prime_768 as BN_get_rfc2409_prime_768 , } ; } }
};
}
