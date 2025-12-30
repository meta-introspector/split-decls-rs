// Generated macro for addrinfo (struct)
macro_rules! Depcrate_syscalls_socket_addrinfoaddrinfo {
() => {
// Module: crate::syscalls::socket::addrinfo
// Provides: {"addrinfo"}
// Dependencies: {}
# [repr (C)] # [derive (Default)] struct addrinfo { ai_flags : Ai , ai_family : i32 , ai_socktype : i32 , ai_protocol : i32 , ai_addrlen : socklen_t , ai_canonname : * mut c_char , ai_addr : * mut sockaddr , ai_next : Option < Box < addrinfo > > , }
};
}
