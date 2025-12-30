// Generated macro for sys_clock_getres (function)
macro_rules! Depcrate_syscalls_timersys_clock_getres {
() => {
// Module: crate::syscalls::timer
// Provides: {"sys_clock_getres"}
// Dependencies: {}
# [doc = " Finds the resolution (or precision) of a clock."] # [doc = ""] # [doc = " This function gets the clock resolution of the clock with `clock_id` and stores it in parameter `res`."] # [doc = " Returns `0` on success, `-EINVAL` otherwise."] # [doc = ""] # [doc = " Supported clocks:"] # [doc = " - `CLOCK_REALTIME`"] # [doc = " - `CLOCK_PROCESS_CPUTIME_ID`"] # [doc = " - `CLOCK_THREAD_CPUTIME_ID`"] # [doc = " - `CLOCK_MONOTONIC`"] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_clock_getres (clock_id : clockid_t , res : * mut timespec) -> i32 { assert ! (! res . is_null () , "sys_clock_getres called with a zero res parameter") ; let result = unsafe { & mut * res } ; match clock_id { CLOCK_REALTIME | CLOCK_PROCESS_CPUTIME_ID | CLOCK_THREAD_CPUTIME_ID | CLOCK_MONOTONIC => { * result = timespec :: from_usec (1) ; 0 } _ => { debug ! ("Called sys_clock_getres for unsupported clock {clock_id}") ; - i32 :: from (Errno :: Inval) } } }
};
}
