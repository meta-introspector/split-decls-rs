// Generated macro for impl_45 (impl)
macro_rules! Depcrate_fseventimpl_45 {
() => {
// Module: crate::fsevent
// Provides: {"impl_45"}
// Dependencies: {}
impl Drop for FsEventWatcher { fn drop (& mut self) { self . stop () ; unsafe { cf :: CFRelease (self . paths) ; } } }
};
}
