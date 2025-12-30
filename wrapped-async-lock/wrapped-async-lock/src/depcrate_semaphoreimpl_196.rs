// Generated macro for impl_196 (impl)
macro_rules! Depcrate_semaphoreimpl_196 {
() => {
// Module: crate::semaphore
// Provides: {"impl_196"}
// Dependencies: {}
impl Drop for SemaphoreGuardArc { fn drop (& mut self) { let opt = self . 0 . take () . unwrap () ; opt . count . fetch_add (1 , Ordering :: AcqRel) ; opt . event . notify (1) ; } }
};
}
