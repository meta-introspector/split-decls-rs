// Generated macro for sys_futex_wake (function)
macro_rules! Depcrate_syscalls_futexsys_futex_wake {
() => {
// Module: crate::syscalls::futex
// Provides: {"sys_futex_wake"}
// Dependencies: {}
# [doc = " Like `synch::futex_wake`, but does extra sanity checks."] # [doc = ""] # [doc = " Returns -EINVAL if `address` is null."] # [doc = " `address` is used only for its address."] # [doc = " It is safe to pass a dangling pointer."] # [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_futex_wake (address : * mut u32 , count : i32) -> i32 { if address . is_null () { return - i32 :: from (Errno :: Inval) ; } synch :: futex_wake (address as * const AtomicU32 , count) }
};
}
