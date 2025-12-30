// Generated macro for sys_errno_location (function)
macro_rules! Depcrate_errnosys_errno_location {
() => {
// Module: crate::errno
// Provides: {"sys_errno_location"}
// Dependencies: {}
# [doc = " Returns the pointer to `errno`."] # [cfg (all (not (any (feature = "common-os" , feature = "nostd")) , not (target_arch = "riscv64") ,))] # [unsafe (no_mangle)] # [linkage = "weak"] pub extern "C" fn sys_errno_location () -> * mut i32 { use core :: cell :: UnsafeCell ; # [thread_local] static ERRNO : UnsafeCell < i32 > = UnsafeCell :: new (0) ; ERRNO . get () }
};
}
