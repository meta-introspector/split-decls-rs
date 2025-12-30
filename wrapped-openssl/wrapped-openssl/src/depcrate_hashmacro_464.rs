// Generated macro for macro_464 (macro)
macro_rules! Depcrate_hashmacro_464 {
() => {
// Module: crate::hash
// Provides: {"macro_464"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , boringssl , libressl382 , awslc))] { use ffi :: { EVP_MD_CTX_free , EVP_MD_CTX_new } ; } else { use ffi :: { EVP_MD_CTX_create as EVP_MD_CTX_new , EVP_MD_CTX_destroy as EVP_MD_CTX_free } ; } }
};
}
