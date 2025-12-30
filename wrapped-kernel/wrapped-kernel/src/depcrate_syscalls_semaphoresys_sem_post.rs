// Generated macro for sys_sem_post (function)
macro_rules! Depcrate_syscalls_semaphoresys_sem_post {
() => {
// Module: crate::syscalls::semaphore
// Provides: {"sys_sem_post"}
// Dependencies: {}
# [doc = " Release a semaphore."] # [doc = ""] # [doc = " This function can be used to allow the next blocked waiter to access this semaphore."] # [doc = " It will notify the next waiter that `sem` is available."] # [doc = " The semaphore is not deallocated after being released."] # [doc = ""] # [doc = " Returns `0` on success, or `-EINVAL` if `sem` is null."] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_sem_post (sem : * mut sem_t) -> i32 { if sem . is_null () { return - i32 :: from (Errno :: Inval) ; } let semaphore = unsafe { & * * sem } ; semaphore . release () ; 0 }
};
}
