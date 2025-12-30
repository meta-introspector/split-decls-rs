// Generated macro for impl_158 (impl)
macro_rules! Depcrate_os_iocpimpl_158 {
() => {
// Module: crate::os::iocp
// Provides: {"impl_158"}
// Dependencies: {}
impl HasAfdInfo for PacketInner { fn afd_info (self : Pin < & Self >) -> Pin < & UnsafeCell < AfdPollInfo > > { match self . project_ref () { PacketInnerProj :: Socket { packet , .. } => packet , _ => unreachable ! () , } } }
};
}
