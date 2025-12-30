// Generated macro for macro_1342 (macro)
macro_rules! Depcrate_versionmacro_1342 {
() => {
// Module: crate::version
// Provides: {"macro_1342"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , libressl))] { use ffi :: { OPENSSL_VERSION , OPENSSL_CFLAGS , OPENSSL_BUILT_ON , OPENSSL_PLATFORM , OPENSSL_DIR , OpenSSL_version_num , OpenSSL_version , } ; } else { use ffi :: { SSLEAY_VERSION as OPENSSL_VERSION , SSLEAY_CFLAGS as OPENSSL_CFLAGS , SSLEAY_BUILT_ON as OPENSSL_BUILT_ON , SSLEAY_PLATFORM as OPENSSL_PLATFORM , SSLEAY_DIR as OPENSSL_DIR , SSLeay as OpenSSL_version_num , SSLeay_version as OpenSSL_version , } ; } }
};
}
