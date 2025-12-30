// Generated macro for ipv6_mreq (struct)
macro_rules! Depcrate_syscalls_socketipv6_mreq {
() => {
// Module: crate::syscalls::socket
// Provides: {"ipv6_mreq"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct ipv6_mreq { pub ipv6mr_multiaddr : in6_addr , pub ipv6mr_interface : u32 , }
};
}
