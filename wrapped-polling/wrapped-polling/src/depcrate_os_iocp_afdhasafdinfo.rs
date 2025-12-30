// Generated macro for HasAfdInfo (trait)
macro_rules! Depcrate_os_iocp_afdHasAfdInfo {
() => {
// Module: crate::os::iocp::afd
// Provides: {"HasAfdInfo"}
// Dependencies: {}
pub (super) trait HasAfdInfo { fn afd_info (self : Pin < & Self >) -> Pin < & UnsafeCell < AfdPollInfo > > ; }
};
}
