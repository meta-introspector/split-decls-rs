// Generated macro for sys_fcntl (function)
macro_rules! Depcrate_syscallssys_fcntl {
() => {
// Module: crate::syscalls
// Provides: {"sys_fcntl"}
// Dependencies: {}
# [doc = " manipulate file descriptor"] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_fcntl (fd : i32 , cmd : i32 , arg : i32) -> i32 { const F_SETFD : i32 = 2 ; const F_GETFL : i32 = 3 ; const F_SETFL : i32 = 4 ; const FD_CLOEXEC : i32 = 1 ; if cmd == F_SETFD && arg == FD_CLOEXEC { 0 } else if cmd == F_GETFL { let obj = get_object (fd) ; obj . map_or_else (| e | - i32 :: from (e) , | v | { block_on (async { v . read () . await . status_flags () . await } , None) . map_or_else (| e | - i32 :: from (e) , | status_flags | status_flags . bits ()) } ,) } else if cmd == F_SETFL { let obj = get_object (fd) ; obj . map_or_else (| e | - i32 :: from (e) , | v | { block_on (async { v . write () . await . set_status_flags (fd :: StatusFlags :: from_bits_retain (arg)) . await } , None ,) . map_or_else (| e | - i32 :: from (e) , | () | 0) } ,) } else { - i32 :: from (Errno :: Inval) } }
};
}
