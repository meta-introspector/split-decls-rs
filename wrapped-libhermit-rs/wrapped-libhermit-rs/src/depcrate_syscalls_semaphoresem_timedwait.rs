// Generated macro for sem_timedwait (function)
macro_rules! Depcrate_syscalls_semaphoresem_timedwait {
() => {
// Module: crate::syscalls::semaphore
// Provides: {"sem_timedwait"}
// Dependencies: {}
unsafe fn sem_timedwait (sem : * mut sem_t , ms : u32) -> i32 { if sem . is_null () { return - i32 :: from (Errno :: Inval) ; } let delay = if ms > 0 { Some (u64 :: from (ms)) } else { None } ; let semaphore = unsafe { & * * sem } ; if semaphore . acquire (delay) { 0 } else { - i32 :: from (Errno :: Time) } }
};
}
