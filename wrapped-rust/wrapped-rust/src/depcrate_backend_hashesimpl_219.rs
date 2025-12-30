// Generated macro for impl_219 (impl)
macro_rules! Depcrate_backend_hashesimpl_219 {
() => {
// Module: crate::backend::hashes
// Provides: {"impl_219"}
// Dependencies: {}
impl Hash { pub (crate) fn update_bytes (& mut self , data : & [u8]) -> CryptographyResult < () > { self . get_mut_ctx () ? . update (data) ? ; Ok (()) } }
};
}
