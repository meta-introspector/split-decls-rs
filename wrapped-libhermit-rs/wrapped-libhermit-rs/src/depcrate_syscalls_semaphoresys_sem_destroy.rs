// Generated macro for sys_sem_destroy (function)
macro_rules! Depcrate_syscalls_semaphoresys_sem_destroy {
() => {
// Module: crate::syscalls::semaphore
// Provides: {"sys_sem_destroy"}
// Dependencies: {}
# [doc = " Destroy and deallocate a semaphore."] # [doc = ""] # [doc = " This function can be used to manually deallocate a semaphore via a reference."] # [doc = ""] # [doc = " Returns `0` on success, `-EINVAL` if `sem` is null."] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_sem_destroy (sem : * mut sem_t) -> i32 { if sem . is_null () { return - i32 :: from (Errno :: Inval) ; } unsafe { drop (Box :: from_raw ((* sem) . cast_mut ())) ; } 0 }
};
}
