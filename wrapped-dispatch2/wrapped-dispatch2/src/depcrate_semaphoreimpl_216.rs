// Generated macro for impl_216 (impl)
macro_rules! Depcrate_semaphoreimpl_216 {
() => {
// Module: crate::semaphore
// Provides: {"impl_216"}
// Dependencies: {}
impl DispatchSemaphoreGuard { # [doc = " Release the [`DispatchSemaphore`]."] pub fn release (self) -> bool { let this = ManuallyDrop :: new (self) ; let result = this . 0 . signal () ; result != 0 } }
};
}
