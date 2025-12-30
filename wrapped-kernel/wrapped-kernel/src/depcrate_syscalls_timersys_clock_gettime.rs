// Generated macro for sys_clock_gettime (function)
macro_rules! Depcrate_syscalls_timersys_clock_gettime {
() => {
// Module: crate::syscalls::timer
// Provides: {"sys_clock_gettime"}
// Dependencies: {}
# [doc = " Get the current time of a clock."] # [doc = ""] # [doc = " Get the current time of the clock with `clock_id` and stores result in parameter `res`."] # [doc = " Returns `0` on success, `-EINVAL` otherwise."] # [doc = ""] # [doc = " Supported clocks:"] # [doc = " - `CLOCK_REALTIME`"] # [doc = " - `CLOCK_MONOTONIC`"] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_clock_gettime (clock_id : clockid_t , tp : * mut timespec) -> i32 { assert ! (! tp . is_null () , "sys_clock_gettime called with a zero tp parameter") ; let result = unsafe { & mut * tp } ; match clock_id { CLOCK_REALTIME => { * result = timespec :: from_usec (arch :: kernel :: systemtime :: now_micros () as i64) ; 0 } CLOCK_MONOTONIC => { * result = timespec :: from_usec (arch :: processor :: get_timer_ticks () as i64) ; 0 } _ => { debug ! ("Called sys_clock_gettime for unsupported clock {clock_id}") ; - i32 :: from (Errno :: Inval) } } }
};
}
