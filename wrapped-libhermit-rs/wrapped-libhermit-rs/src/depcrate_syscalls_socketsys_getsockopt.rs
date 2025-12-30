// Generated macro for sys_getsockopt (function)
macro_rules! Depcrate_syscalls_socketsys_getsockopt {
() => {
// Module: crate::syscalls::socket
// Provides: {"sys_getsockopt"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_getsockopt (fd : i32 , level : i32 , optname : i32 , optval : * mut c_void , optlen : * mut socklen_t ,) -> i32 { let Ok (Ok (level)) = u8 :: try_from (level) . map (Ipproto :: try_from) else { return - i32 :: from (Errno :: Inval) ; } ; debug ! ("sys_getsockopt: {fd}, level {level:?}, optname {optname}") ; if level == Ipproto :: Tcp && optname == TCP_NODELAY { if optval . is_null () || optlen . is_null () { return - i32 :: from (Errno :: Inval) ; } let optval = unsafe { & mut * optval . cast :: < i32 > () } ; let optlen = unsafe { & mut * optlen } ; let obj = get_object (fd) ; obj . map_or_else (| e | - i32 :: from (e) , | v | { block_on (async { v . read () . await . getsockopt (SocketOption :: TcpNoDelay) . await } , None ,) . map_or_else (| e | - i32 :: from (e) , | value | { if value { * optval = 1 ; } else { * optval = 0 ; } * optlen = core :: mem :: size_of :: < i32 > () . try_into () . unwrap () ; 0 } ,) } ,) } else { - i32 :: from (Errno :: Inval) } }
};
}
