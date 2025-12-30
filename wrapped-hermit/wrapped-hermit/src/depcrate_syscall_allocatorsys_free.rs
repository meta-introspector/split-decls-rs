// Generated macro for sys_free (function)
macro_rules! Depcrate_syscall_allocatorsys_free {
() => {
// Module: crate::syscall::allocator
// Provides: {"sys_free"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_free (ptr : * mut u8 , size : usize , align : usize) { let layout = Layout :: from_size_align (size , align) . unwrap () ; unsafe { ALLOC . lock () . free (NonNull :: new_unchecked (ptr) , layout) ; } }
};
}
