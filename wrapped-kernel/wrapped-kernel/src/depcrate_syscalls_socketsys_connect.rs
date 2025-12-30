// Generated macro for sys_connect (function)
macro_rules! Depcrate_syscalls_socketsys_connect {
() => {
// Module: crate::syscalls::socket
// Provides: {"sys_connect"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_connect (fd : i32 , name : * const sockaddr , namelen : socklen_t) -> i32 { if name . is_null () { return - i32 :: from (Errno :: Inval) ; } let Ok (sa_family) = (unsafe { Af :: try_from ((* name) . sa_family) }) else { return - i32 :: from (Errno :: Inval) ; } ; let endpoint = match sa_family { # [cfg (feature = "net")] Af :: Inet => { if namelen < u32 :: try_from (size_of :: < sockaddr_in > ()) . unwrap () { return - i32 :: from (Errno :: Inval) ; } Endpoint :: Ip (IpEndpoint :: from (unsafe { * name . cast :: < sockaddr_in > () })) } # [cfg (feature = "net")] Af :: Inet6 => { if namelen < u32 :: try_from (size_of :: < sockaddr_in6 > ()) . unwrap () { return - i32 :: from (Errno :: Inval) ; } Endpoint :: Ip (IpEndpoint :: from (unsafe { * name . cast :: < sockaddr_in6 > () })) } # [cfg (feature = "vsock")] Af :: Vsock => { if namelen < u32 :: try_from (size_of :: < sockaddr_vm > ()) . unwrap () { return - i32 :: from (Errno :: Inval) ; } Endpoint :: Vsock (VsockEndpoint :: from (unsafe { * name . cast :: < sockaddr_vm > () })) } _ => { return - i32 :: from (Errno :: Inval) ; } } ; let obj = get_object (fd) ; obj . map_or_else (| e | - i32 :: from (e) , | v | { block_on (async { v . write () . await . connect (endpoint) . await } , None) . map_or_else (| e | - i32 :: from (e) , | () | 0) } ,) }
};
}
