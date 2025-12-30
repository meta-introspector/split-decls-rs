// Generated macro for sys_getpeername (function)
macro_rules! Depcrate_syscalls_socketsys_getpeername {
() => {
// Module: crate::syscalls::socket
// Provides: {"sys_getpeername"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_getpeername (fd : i32 , addr : * mut sockaddr , addrlen : * mut socklen_t ,) -> i32 { let obj = get_object (fd) ; obj . map_or_else (| e | - i32 :: from (e) , | v | { if let Ok (Some (endpoint)) = block_on (async { v . read () . await . getpeername () . await } , None) { if ! addr . is_null () && ! addrlen . is_null () { let addrlen = unsafe { & mut * addrlen } ; match endpoint { # [cfg (feature = "net")] Endpoint :: Ip (endpoint) => match endpoint . addr { IpAddress :: Ipv4 (_) => { if * addrlen >= u32 :: try_from (size_of :: < sockaddr_in > ()) . unwrap () { let addr = unsafe { & mut * addr . cast () } ; * addr = sockaddr_in :: from (endpoint) ; * addrlen = size_of :: < sockaddr_in > () . try_into () . unwrap () ; } else { return - i32 :: from (Errno :: Inval) ; } } IpAddress :: Ipv6 (_) => { if * addrlen >= u32 :: try_from (size_of :: < sockaddr_in6 > ()) . unwrap () { let addr = unsafe { & mut * addr . cast () } ; * addr = sockaddr_in6 :: from (endpoint) ; * addrlen = size_of :: < sockaddr_in6 > () . try_into () . unwrap () ; } else { return - i32 :: from (Errno :: Inval) ; } } } , # [cfg (feature = "vsock")] Endpoint :: Vsock (_) => { if * addrlen >= u32 :: try_from (size_of :: < sockaddr_vm > ()) . unwrap () { warn ! ("unsupported device") ; } else { return - i32 :: from (Errno :: Inval) ; } } } } else { return - i32 :: from (Errno :: Inval) ; } } 0 } ,) }
};
}
