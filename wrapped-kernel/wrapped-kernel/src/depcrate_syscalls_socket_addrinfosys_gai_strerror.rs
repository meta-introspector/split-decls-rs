// Generated macro for sys_gai_strerror (function)
macro_rules! Depcrate_syscalls_socket_addrinfosys_gai_strerror {
() => {
// Module: crate::syscalls::socket::addrinfo
// Provides: {"sys_gai_strerror"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_gai_strerror (ecode : i32) -> * const c_char { let Ok (ecode) = Eai :: try_from (ecode) else { return c"Unknown error" . as_ptr () ; } ; let s = match ecode { Eai :: Again => c"Try again" , Eai :: Badflags => c"Invalid flags" , Eai :: Fail => c"Non-recoverable error" , Eai :: Family => c"Unrecognized address family or invalid length" , Eai :: Memory => c"Out of memory" , Eai :: Nodata => c"Name has no usable address" , Eai :: Noname => c"Name does not resolve" , Eai :: Service => c"Unrecognized service" , Eai :: Socktype => c"Unrecognized socket type" , Eai :: System => c"System error" , Eai :: Overflow => c"Overflow" , } ; s . as_ptr () }
};
}
