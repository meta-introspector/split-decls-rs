// Generated macro for use_24 (use)
macro_rules! Depcrate_fipsuse_24 {
() => {
// Module: crate::fips
// Provides: {"use_24"}
// Dependencies: {}
# [cfg (not (any (CRYPTOGRAPHY_IS_LIBRESSL , CRYPTOGRAPHY_IS_BORINGSSL , CRYPTOGRAPHY_IS_AWSLC)))] use openssl_sys as ffi ;
};
}
