// Generated macro for sys_realloc (function)
macro_rules! Depcrate_syscall_allocatorsys_realloc {
() => {
// Module: crate::syscall::allocator
// Provides: {"sys_realloc"}
// Dependencies: {}
# [no_mangle] pub extern "C" fn sys_realloc (ptr : * mut u8 , size : usize , align : usize , new_size : usize) -> * mut u8 { unsafe { let layout = Layout :: from_size_align (size , align) . unwrap () ; if new_size > size { ALLOC . lock () . grow (NonNull :: new_unchecked (ptr) , layout , new_size) . unwrap () . as_mut () } else { ALLOC . lock () . shrink (NonNull :: new_unchecked (ptr) , layout , new_size) ; ptr } } }
};
}
