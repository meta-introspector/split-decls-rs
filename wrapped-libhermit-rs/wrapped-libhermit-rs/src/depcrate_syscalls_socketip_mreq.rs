// Generated macro for ip_mreq (struct)
macro_rules! Depcrate_syscalls_socketip_mreq {
() => {
// Module: crate::syscalls::socket
// Provides: {"ip_mreq"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct ip_mreq { pub imr_multiaddr : in_addr , pub imr_interface : in_addr , }
};
}
