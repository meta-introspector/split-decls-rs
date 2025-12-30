// Generated macro for sys_sem_trywait (function)
macro_rules! Depcrate_syscalls_semaphoresys_sem_trywait {
() => {
// Module: crate::syscalls::semaphore
// Provides: {"sys_sem_trywait"}
// Dependencies: {}
# [doc = " Try to acquire a lock on a semaphore."] # [doc = ""] # [doc = " This function does not block if the acquire fails."] # [doc = " If the acquire fails (i.e. the semaphore's count is already 0), the function returns immediately."] # [doc = ""] # [doc = " Returns `0` on lock acquire, `-EINVAL` if `sem` is null, or `-ECANCELED` if the decrement fails."] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_sem_trywait (sem : * mut sem_t) -> i32 { if sem . is_null () { return - i32 :: from (Errno :: Inval) ; } let semaphore = unsafe { & * * sem } ; if semaphore . try_acquire () { 0 } else { - i32 :: from (Errno :: Canceled) } }
};
}
