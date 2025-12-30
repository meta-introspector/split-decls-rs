// Generated macro for sockaddr_in6 (struct)
macro_rules! Depcrate_syscalls_socketsockaddr_in6 {
() => {
// Module: crate::syscalls::socket
// Provides: {"sockaddr_in6"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Default , Copy , Clone)] pub struct sockaddr_in6 { pub sin6_len : u8 , pub sin6_family : sa_family_t , pub sin6_port : in_port_t , pub sin6_flowinfo : u32 , pub sin6_addr : in6_addr , pub sin6_scope_id : u32 , }
};
}
