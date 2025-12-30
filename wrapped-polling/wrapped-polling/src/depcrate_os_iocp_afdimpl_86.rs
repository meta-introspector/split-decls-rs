// Generated macro for impl_86 (impl)
macro_rules! Depcrate_os_iocp_afdimpl_86 {
() => {
// Module: crate::os::iocp::afd
// Provides: {"impl_86"}
// Dependencies: {}
unsafe impl < T > Completion for IoStatusBlock < T > { fn try_lock (self : Pin < & Self >) -> bool { ! self . in_use . swap (true , Ordering :: SeqCst) } unsafe fn unlock (self : Pin < & Self >) { self . in_use . store (false , Ordering :: SeqCst) ; } }
};
}
