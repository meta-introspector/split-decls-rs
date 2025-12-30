// Generated macro for impl_193 (impl)
macro_rules! Depcrate_semaphoreimpl_193 {
() => {
// Module: crate::semaphore
// Provides: {"impl_193"}
// Dependencies: {}
impl Drop for SemaphoreGuard < '_ > { fn drop (& mut self) { self . 0 . count . fetch_add (1 , Ordering :: AcqRel) ; self . 0 . event . notify (1) ; } }
};
}
