// Generated macro for macro_828 (macro)
macro_rules! Depcrate_signmacro_828 {
() => {
// Module: crate::sign
// Provides: {"macro_828"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , libressl382))] { use ffi :: { EVP_MD_CTX_free , EVP_MD_CTX_new } ; } else { use ffi :: { EVP_MD_CTX_create as EVP_MD_CTX_new , EVP_MD_CTX_destroy as EVP_MD_CTX_free } ; } }
};
}
