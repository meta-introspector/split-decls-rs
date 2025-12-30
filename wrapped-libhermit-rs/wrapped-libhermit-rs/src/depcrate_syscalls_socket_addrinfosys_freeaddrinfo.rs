// Generated macro for sys_freeaddrinfo (function)
macro_rules! Depcrate_syscalls_socket_addrinfosys_freeaddrinfo {
() => {
// Module: crate::syscalls::socket::addrinfo
// Provides: {"sys_freeaddrinfo"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_freeaddrinfo (ai : Option < Box < addrinfo > >) { drop (ai) ; }
};
}
