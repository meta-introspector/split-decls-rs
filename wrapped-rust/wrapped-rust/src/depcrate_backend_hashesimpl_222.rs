// Generated macro for impl_222 (impl)
macro_rules! Depcrate_backend_hashesimpl_222 {
() => {
// Module: crate::backend::hashes
// Provides: {"impl_222"}
// Dependencies: {}
impl XOFHash { pub (crate) fn update_bytes (& mut self , data : & [u8]) -> CryptographyResult < () > { self . ctx . update (data) ? ; Ok (()) } }
};
}
