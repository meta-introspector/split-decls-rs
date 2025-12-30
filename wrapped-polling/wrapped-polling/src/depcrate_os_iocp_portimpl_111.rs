// Generated macro for impl_111 (impl)
macro_rules! Depcrate_os_iocp_portimpl_111 {
() => {
// Module: crate::os::iocp::port
// Provides: {"impl_111"}
// Dependencies: {}
impl < T > Drop for IoCompletionPort < T > { fn drop (& mut self) { unsafe { CloseHandle (self . handle) ; } } }
};
}
