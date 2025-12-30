// Generated macro for impl_74 (impl)
macro_rules! Depcrate_os_iocp_afdimpl_74 {
() => {
// Module: crate::os::iocp::afd
// Provides: {"impl_74"}
// Dependencies: {}
impl < T > Drop for Afd < T > { fn drop (& mut self) { unsafe { CloseHandle (self . handle) ; } } }
};
}
