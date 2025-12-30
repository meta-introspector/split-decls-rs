// Generated macro for impl_477 (impl)
macro_rules! Depcrate_hashimpl_477 {
() => {
// Module: crate::hash
// Provides: {"impl_477"}
// Dependencies: {}
impl Drop for Hasher { fn drop (& mut self) { unsafe { if self . state != Finalized { drop (self . finish ()) ; } EVP_MD_CTX_free (self . ctx) ; } } }
};
}
