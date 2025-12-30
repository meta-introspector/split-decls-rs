// Generated macro for sockaddr (struct)
macro_rules! Depcrate_syscalls_socketsockaddr {
() => {
// Module: crate::syscalls::socket
// Provides: {"sockaddr"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Default , Copy , Clone)] pub struct sockaddr { pub sa_len : u8 , pub sa_family : sa_family_t , pub sa_data : [c_char ; 14] , }
};
}
