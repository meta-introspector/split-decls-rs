// Generated macro for sys_setsockopt (function)
macro_rules! Depcrate_syscalls_socketsys_setsockopt {
() => {
// Module: crate::syscalls::socket
// Provides: {"sys_setsockopt"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_setsockopt (fd : i32 , level : i32 , optname : i32 , optval : * const c_void , optlen : socklen_t ,) -> i32 { if level == SOL_SOCKET && optname == SO_REUSEADDR { return 0 ; } let Ok (Ok (level)) = u8 :: try_from (level) . map (Ipproto :: try_from) else { return - i32 :: from (Errno :: Inval) ; } ; debug ! ("sys_setsockopt: {fd}, level {level:?}, optname {optname}") ; if level == Ipproto :: Tcp && optname == TCP_NODELAY && optlen == u32 :: try_from (size_of :: < i32 > ()) . unwrap () { if optval . is_null () { return - i32 :: from (Errno :: Inval) ; } let value = unsafe { * optval . cast :: < i32 > () } ; let obj = get_object (fd) ; obj . map_or_else (| e | - i32 :: from (e) , | v | { block_on (async { v . read () . await . setsockopt (SocketOption :: TcpNoDelay , value != 0) . await } , None ,) . map_or_else (| e | - i32 :: from (e) , | () | 0) } ,) } else { - i32 :: from (Errno :: Inval) } }
};
}
