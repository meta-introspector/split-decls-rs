// Generated macro for sockaddr_un (struct)
macro_rules! Depcrate_syscalls_socketsockaddr_un {
() => {
// Module: crate::syscalls::socket
// Provides: {"sockaddr_un"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct sockaddr_un { pub sun_len : u8 , pub sun_family : sa_family_t , pub sun_path : [c_char ; 104] , }
};
}
