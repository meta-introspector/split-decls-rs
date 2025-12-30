// Generated macro for sys_clock_nanosleep (function)
macro_rules! Depcrate_syscalls_timersys_clock_nanosleep {
() => {
// Module: crate::syscalls::timer
// Provides: {"sys_clock_nanosleep"}
// Dependencies: {}
# [doc = " Sleep a clock for a specified number of nanoseconds."] # [doc = ""] # [doc = " The requested time (in nanoseconds) must be greater than 0 and less than 1,000,000."] # [doc = ""] # [doc = " Returns `0` on success, `-EINVAL` otherwise."] # [doc = ""] # [doc = " Supported clocks:"] # [doc = " - `CLOCK_REALTIME`"] # [doc = " - `CLOCK_MONOTONIC`"] # [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_clock_nanosleep (clock_id : clockid_t , flags : i32 , rqtp : * const timespec , _rmtp : * mut timespec ,) -> i32 { assert ! (! rqtp . is_null () , "sys_clock_nanosleep called with a zero rqtp parameter") ; let requested_time = unsafe { & * rqtp } ; if requested_time . tv_sec < 0 || requested_time . tv_nsec > 999_999_999 { debug ! ("sys_clock_nanosleep called with an invalid requested time, returning -EINVAL") ; return - i32 :: from (Errno :: Inval) ; } match clock_id { CLOCK_REALTIME | CLOCK_MONOTONIC => { let mut microseconds = (requested_time . tv_sec as u64) * 1_000_000 + (requested_time . tv_nsec as u64) / 1_000 ; if flags & TIMER_ABSTIME > 0 { if clock_id == CLOCK_REALTIME { microseconds -= arch :: kernel :: systemtime :: now_micros () ; } else { microseconds -= arch :: processor :: get_timer_ticks () ; } } usleep (microseconds) ; 0 } _ => - i32 :: from (Errno :: Inval) , } }
};
}
