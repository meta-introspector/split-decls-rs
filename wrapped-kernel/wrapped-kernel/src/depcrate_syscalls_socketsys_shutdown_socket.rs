// Generated macro for sys_shutdown_socket (function)
macro_rules! Depcrate_syscalls_socketsys_shutdown_socket {
() => {
// Module: crate::syscalls::socket
// Provides: {"sys_shutdown_socket"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub extern "C" fn sys_shutdown_socket (fd : i32 , how : i32) -> i32 { shutdown (fd , how) }
};
}
