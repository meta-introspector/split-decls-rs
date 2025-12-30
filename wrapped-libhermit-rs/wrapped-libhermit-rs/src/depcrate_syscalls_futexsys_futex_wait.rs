// Generated macro for sys_futex_wait (function)
macro_rules! Depcrate_syscalls_futexsys_futex_wait {
() => {
// Module: crate::syscalls::futex
// Provides: {"sys_futex_wait"}
// Dependencies: {}
# [doc = " Like `synch::futex_wait`, but does extra sanity checks and takes a `timespec`."] # [doc = ""] # [doc = " Returns -EINVAL if"] # [doc = " * `address` is null"] # [doc = " * `timeout` is negative"] # [doc = " * `flags` contains unknown flags"] # [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_futex_wait (address : * mut u32 , expected : u32 , timeout : * const timespec , flags : u32 ,) -> i32 { if address . is_null () { return - i32 :: from (Errno :: Inval) ; } let address = unsafe { & * (address as * const AtomicU32) } ; let timeout = if timeout . is_null () { None } else { match unsafe { timeout . read () . into_usec () } { Some (usec) if usec >= 0 => Some (usec as u64) , _ => return - i32 :: from (Errno :: Inval) , } } ; let Some (flags) = Flags :: from_bits (flags) else { return - i32 :: from (Errno :: Inval) ; } ; synch :: futex_wait (address , expected , timeout , flags) }
};
}
