// Generated macro for addrinfo (struct)
macro_rules! Depcrateaddrinfo {
() => {
// Module: crate
// Provides: {"addrinfo"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Copy , Clone)] pub struct addrinfo { pub ai_flags : i32 , pub ai_family : i32 , pub ai_socktype : i32 , pub ai_protocol : i32 , pub ai_addrlen : socklen_t , pub ai_canonname : * mut c_char , pub ai_addr : * mut sockaddr , pub ai_next : * mut addrinfo , }
};
}
