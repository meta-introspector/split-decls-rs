// Generated macro for impl_118 (impl)
macro_rules! Depcrate_os_iocp_portimpl_118 {
() => {
// Module: crate::os::iocp::port
// Provides: {"impl_118"}
// Dependencies: {}
impl < T : CompletionHandle > Drop for OverlappedEntry < T > { fn drop (& mut self) { drop (unsafe { self . packet () }) ; } }
};
}
