// Generated macro for impl_83 (impl)
macro_rules! Depcrate_os_iocp_afdimpl_83 {
() => {
// Module: crate::os::iocp::afd
// Provides: {"impl_83"}
// Dependencies: {}
impl < T : HasAfdInfo > HasAfdInfo for IoStatusBlock < T > { fn afd_info (self : Pin < & Self >) -> Pin < & UnsafeCell < AfdPollInfo > > { self . project_ref () . data . afd_info () } }
};
}
