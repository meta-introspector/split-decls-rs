// Generated macro for macro_749 (macro)
macro_rules! Depcrate_pkey_ctxmacro_749 {
() => {
// Module: crate::pkey_ctx
// Provides: {"macro_749"}
// Dependencies: {}
generic_foreign_type_and_impl_send_sync ! { type CType = ffi :: EVP_PKEY_CTX ; fn drop = ffi :: EVP_PKEY_CTX_free ; # [doc = " A context object which can perform asymmetric cryptography operations."] pub struct PkeyCtx < T >; # [doc = " A reference to a [`PkeyCtx`]."] pub struct PkeyCtxRef < T >; }
};
}
