// Generated macro for macro_235 (macro)
macro_rules! Depcrate_cipher_ctxmacro_235 {
() => {
// Module: crate::cipher_ctx
// Provides: {"macro_235"}
// Dependencies: {}
# [cfg (ossl102)] bitflags ! { # [doc = " Flags for `EVP_CIPHER_CTX`."] pub struct CipherCtxFlags : c_int { # [doc = " The flag used to opt into AES key wrap ciphers."] const FLAG_WRAP_ALLOW = ffi :: EVP_CIPHER_CTX_FLAG_WRAP_ALLOW ; } }
};
}
