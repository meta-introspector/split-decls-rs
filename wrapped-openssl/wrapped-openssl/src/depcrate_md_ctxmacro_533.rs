// Generated macro for macro_533 (macro)
macro_rules! Depcrate_md_ctxmacro_533 {
() => {
// Module: crate::md_ctx
// Provides: {"macro_533"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , boringssl , libressl382 , awslc))] { use ffi :: { EVP_MD_CTX_free , EVP_MD_CTX_new } ; } else { use ffi :: { EVP_MD_CTX_create as EVP_MD_CTX_new , EVP_MD_CTX_destroy as EVP_MD_CTX_free } ; } }
};
}
