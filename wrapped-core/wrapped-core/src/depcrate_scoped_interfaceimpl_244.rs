// Generated macro for impl_244 (impl)
macro_rules! Depcrate_scoped_interfaceimpl_244 {
() => {
// Module: crate::scoped_interface
// Provides: {"impl_244"}
// Dependencies: {}
impl < T : Interface > Drop for ScopedInterface < '_ , T > { fn drop (& mut self) { unsafe { let _ = Box :: from_raw (self . interface . as_raw () as * const _ as * mut ScopedHeap) ; } } }
};
}
