// Generated macro for sys_clock_settime (function)
macro_rules! Depcrate_syscalls_timersys_clock_settime {
() => {
// Module: crate::syscalls::timer
// Provides: {"sys_clock_settime"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_clock_settime (_clock_id : clockid_t , _tp : * const timespec) -> i32 { debug ! ("sys_clock_settime is unimplemented, returning -EINVAL") ; - i32 :: from (Errno :: Inval) }
};
}
