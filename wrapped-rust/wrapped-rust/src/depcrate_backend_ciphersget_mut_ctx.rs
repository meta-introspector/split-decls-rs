// Generated macro for get_mut_ctx (function)
macro_rules! Depcrate_backend_ciphersget_mut_ctx {
() => {
// Module: crate::backend::ciphers
// Provides: {"get_mut_ctx"}
// Dependencies: {}
fn get_mut_ctx (ctx : Option < & mut CipherContext >) -> CryptographyResult < & mut CipherContext > { ctx . ok_or_else (exceptions :: already_finalized_error) }
};
}
