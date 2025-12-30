// Generated macro for sys_sem_init (function)
macro_rules! Depcrate_syscalls_semaphoresys_sem_init {
() => {
// Module: crate::syscalls::semaphore
// Provides: {"sys_sem_init"}
// Dependencies: {}
# [doc = " Create a new, unnamed semaphore."] # [doc = ""] # [doc = " This function can be used to get the raw memory location of a semaphore."] # [doc = ""] # [doc = " Stores the raw memory location of the new semaphore in parameter `sem`."] # [doc = " Returns `0` on success, `-EINVAL` if `sem` is null."] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_sem_init (sem : * mut sem_t , pshared : i32 , value : u32) -> i32 { if sem . is_null () || pshared != 0 { return - i32 :: from (Errno :: Inval) ; } let boxed_semaphore = Box :: new (Semaphore :: new (value as isize)) ; unsafe { * sem = Box :: into_raw (boxed_semaphore) ; } 0 }
};
}
