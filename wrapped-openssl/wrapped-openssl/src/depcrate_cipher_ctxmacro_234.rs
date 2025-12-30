// Generated macro for macro_234 (macro)
macro_rules! Depcrate_cipher_ctxmacro_234 {
() => {
// Module: crate::cipher_ctx
// Provides: {"macro_234"}
// Dependencies: {}
foreign_type_and_impl_send_sync ! { type CType = ffi :: EVP_CIPHER_CTX ; fn drop = ffi :: EVP_CIPHER_CTX_free ; # [doc = " A context object used to perform symmetric encryption operations."] pub struct CipherCtx ; # [doc = " A reference to a [`CipherCtx`]."] pub struct CipherCtxRef ; }
};
}
