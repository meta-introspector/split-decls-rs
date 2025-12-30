// Generated macro for impl_409 (impl)
macro_rules! Depcrate_driverimpl_409 {
() => {
// Module: crate::driver
// Provides: {"impl_409"}
// Dependencies: {}
impl Drop for TimingGuard { fn drop (& mut self) { self . inner . take () ; unsafe { std :: mem :: ManuallyDrop :: drop (& mut self . profiler) ; } } }
};
}
