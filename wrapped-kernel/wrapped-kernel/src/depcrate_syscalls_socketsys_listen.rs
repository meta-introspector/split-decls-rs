// Generated macro for sys_listen (function)
macro_rules! Depcrate_syscalls_socketsys_listen {
() => {
// Module: crate::syscalls::socket
// Provides: {"sys_listen"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_listen (fd : i32 , backlog : i32) -> i32 { let obj = get_object (fd) ; obj . map_or_else (| e | - i32 :: from (e) , | v | { block_on (async { v . write () . await . listen (backlog) . await } , None) . map_or_else (| e | - i32 :: from (e) , | () | 0) } ,) }
};
}
