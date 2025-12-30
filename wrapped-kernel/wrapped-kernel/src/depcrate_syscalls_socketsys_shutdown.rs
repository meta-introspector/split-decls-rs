// Generated macro for sys_shutdown (function)
macro_rules! Depcrate_syscalls_socketsys_shutdown {
() => {
// Module: crate::syscalls::socket
// Provides: {"sys_shutdown"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_shutdown (sockfd : i32 , how : i32) -> i32 { shutdown (sockfd , how) }
};
}
