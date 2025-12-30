// Generated macro for sys_nanosleep (function)
macro_rules! Depcrate_syscalls_taskssys_nanosleep {
() => {
// Module: crate::syscalls::tasks
// Provides: {"sys_nanosleep"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_nanosleep (rqtp : * const timespec , _rmtp : * mut timespec) -> i32 { assert ! (! rqtp . is_null () , "sys_nanosleep called with a zero rqtp parameter") ; let requested_time = unsafe { & * rqtp } ; if requested_time . tv_sec < 0 || requested_time . tv_nsec > 999_999_999 { debug ! ("sys_nanosleep called with an invalid requested time, returning -EINVAL") ; return - i32 :: from (Errno :: Inval) ; } let microseconds = (requested_time . tv_sec as u64) * 1_000_000 + (requested_time . tv_nsec as u64) / 1_000 ; usleep (microseconds) ; 0 }
};
}
