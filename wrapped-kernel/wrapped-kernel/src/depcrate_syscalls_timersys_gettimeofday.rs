// Generated macro for sys_gettimeofday (function)
macro_rules! Depcrate_syscalls_timersys_gettimeofday {
() => {
// Module: crate::syscalls::timer
// Provides: {"sys_gettimeofday"}
// Dependencies: {}
# [doc = " Get the system's clock time."] # [doc = ""] # [doc = " This function gets the current time based on the wallclock time when booted up, plus current timer ticks."] # [doc = " Returns `0` on success, `-EINVAL` otherwise."] # [doc = ""] # [doc = " **Parameter `tz` should be set to `0` since tz is obsolete.**"] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_gettimeofday (tp : * mut timeval , tz : usize) -> i32 { if let Some (result) = unsafe { tp . as_mut () } { let microseconds = arch :: kernel :: systemtime :: now_micros () ; * result = timeval :: from_usec (microseconds as i64) ; } if tz > 0 { debug ! ("The tz parameter in sys_gettimeofday is unimplemented, returning -EINVAL") ; return - i32 :: from (Errno :: Inval) ; } 0 }
};
}
