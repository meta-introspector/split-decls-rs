// Generated macro for impl_82 (impl)
macro_rules! Depcrate_os_iocp_afdimpl_82 {
() => {
// Module: crate::os::iocp::afd
// Provides: {"impl_82"}
// Dependencies: {}
impl < T > IoStatusBlock < T > { pub (super) fn iosb (self : Pin < & Self >) -> & UnsafeCell < IO_STATUS_BLOCK > { self . project_ref () . iosb } pub (super) fn data (self : Pin < & Self >) -> Pin < & T > { self . project_ref () . data } }
};
}
