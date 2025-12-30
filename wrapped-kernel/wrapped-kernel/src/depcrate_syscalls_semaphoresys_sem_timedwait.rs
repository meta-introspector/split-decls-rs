// Generated macro for sys_sem_timedwait (function)
macro_rules! Depcrate_syscalls_semaphoresys_sem_timedwait {
() => {
// Module: crate::syscalls::semaphore
// Provides: {"sys_sem_timedwait"}
// Dependencies: {}
# [doc = " Try to acquire a lock on a semaphore."] # [doc = ""] # [doc = " Blocks until semaphore is acquired or until specified time passed"] # [doc = ""] # [doc = " Returns `0` on lock acquire, `-EINVAL` if sem is null, or `-ETIME` on timeout."] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_sem_timedwait (sem : * mut sem_t , ts : * const timespec) -> i32 { if ts . is_null () { unsafe { sem_timedwait (sem , 0) } } else { let mut current_ts = timespec :: default () ; unsafe { sys_clock_gettime (CLOCK_REALTIME , & raw mut current_ts) ; let ts = & * ts ; let ms : i64 = (ts . tv_sec - current_ts . tv_sec) * 1000 + (i64 :: from (ts . tv_nsec) - i64 :: from (current_ts . tv_nsec)) / 1_000_000 ; if ms > 0 { sem_timedwait (sem , ms . try_into () . unwrap ()) } else { 0 } } } }
};
}
