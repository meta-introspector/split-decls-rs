// Generated macro for sys_send (function)
macro_rules! Depcrate_syscalls_socketsys_send {
() => {
// Module: crate::syscalls::socket
// Provides: {"sys_send"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_send (s : i32 , mem : * const c_void , len : usize , _flags : i32) -> isize { unsafe { super :: write (s , mem . cast () , len) } }
};
}
