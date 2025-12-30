// Generated macro for sys_alloc (function)
macro_rules! Depcrate_syscallssys_alloc {
() => {
// Module: crate::syscalls
// Provides: {"sys_alloc"}
// Dependencies: {}
# [doc = " Interface to allocate memory from system heap"] # [doc = ""] # [doc = " # Errors"] # [doc = " Returning a null pointer indicates that either memory is exhausted or"] # [doc = " `size` and `align` do not meet this allocator's size or alignment constraints."] # [doc = ""] # [cfg (all (target_os = "none" , not (feature = "common-os")))] # [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_alloc (size : usize , align : usize) -> * mut u8 { let layout_res = Layout :: from_size_align (size , align) ; if layout_res . is_err () || size == 0 { warn ! ("__sys_alloc called with size {size:#x}, align {align:#x} is an invalid layout!") ; return core :: ptr :: null_mut () ; } let layout = layout_res . unwrap () ; let ptr = unsafe { ALLOCATOR . alloc (layout) } ; trace ! ("__sys_alloc: allocate memory at {ptr:p} (size {size:#x}, align {align:#x})") ; ptr }
};
}
