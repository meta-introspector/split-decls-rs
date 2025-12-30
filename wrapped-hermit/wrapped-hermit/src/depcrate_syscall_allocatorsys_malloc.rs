// Generated macro for sys_malloc (function)
macro_rules! Depcrate_syscall_allocatorsys_malloc {
() => {
// Module: crate::syscall::allocator
// Provides: {"sys_malloc"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_malloc (size : usize , align : usize) -> * mut u8 { let layout = Layout :: from_size_align (size , align) . unwrap () ; unsafe { ALLOC . lock () . malloc (layout) . unwrap () . as_mut () } }
};
}
