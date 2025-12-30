// Generated macro for impl_192 (impl)
macro_rules! Depcrate_semaphoreimpl_192 {
() => {
// Module: crate::semaphore
// Provides: {"impl_192"}
// Dependencies: {}
impl SemaphoreGuard < '_ > { # [doc = " Drops the guard _without_ releasing the acquired permit."] # [inline] pub fn forget (self) { mem :: forget (self) ; } }
};
}
