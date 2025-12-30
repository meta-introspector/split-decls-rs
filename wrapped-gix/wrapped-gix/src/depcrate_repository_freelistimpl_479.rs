// Generated macro for impl_479 (impl)
macro_rules! Depcrate_repository_freelistimpl_479 {
() => {
// Module: crate::repository::freelist
// Provides: {"impl_479"}
// Dependencies: {}
impl Drop for Buffer < '_ > { fn drop (& mut self) { self . repo . reuse_buffer (& mut self . inner) ; } }
};
}
