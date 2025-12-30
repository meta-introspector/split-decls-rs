// Generated macro for impl_117 (impl)
macro_rules! Depcrate_os_iocp_portimpl_117 {
() => {
// Module: crate::os::iocp::port
// Provides: {"impl_117"}
// Dependencies: {}
impl < T : CompletionHandle > OverlappedEntry < T > { # [doc = " Convert into the completion packet."] pub (super) fn into_packet (self) -> T { let packet = unsafe { self . packet () } ; std :: mem :: forget (self) ; packet } # [doc = " Get the packet reference that this entry refers to."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function should only be called once, since it moves"] # [doc = " out the `T` from the `OVERLAPPED_ENTRY`."] unsafe fn packet (& self) -> T { let packet = T :: from_ptr (self . entry . lpOverlapped) ; packet . get () . unlock () ; packet } }
};
}
