// Generated macro for impl_216 (impl)
macro_rules! Depcrate_backend_hashesimpl_216 {
() => {
// Module: crate::backend::hashes
// Provides: {"impl_216"}
// Dependencies: {}
impl Hash { fn get_ctx (& self) -> CryptographyResult < & openssl :: hash :: Hasher > { if let Some (ctx) = self . ctx . as_ref () { return Ok (ctx) ; } ; Err (exceptions :: already_finalized_error ()) } fn get_mut_ctx (& mut self) -> CryptographyResult < & mut openssl :: hash :: Hasher > { if let Some (ctx) = self . ctx . as_mut () { return Ok (ctx) ; } Err (exceptions :: already_finalized_error ()) } }
};
}
