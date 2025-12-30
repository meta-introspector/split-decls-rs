// Generated macro for macro_233 (macro)
macro_rules! Depcrate_cipher_ctxmacro_233 {
() => {
// Module: crate::cipher_ctx
// Provides: {"macro_233"}
// Dependencies: {}
cfg_if ! { if # [cfg (ossl300)] { use ffi :: EVP_CIPHER_CTX_get0_cipher ; } else { use ffi :: EVP_CIPHER_CTX_cipher as EVP_CIPHER_CTX_get0_cipher ; } }
};
}
