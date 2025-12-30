// Generated macro for sys_malloc (function)
macro_rules! Depcrate_syscallssys_malloc {
() => {
// Module: crate::syscalls
// Provides: {"sys_malloc"}
// Dependencies: {}
# [cfg (all (target_os = "none" , not (feature = "common-os")))] # [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_malloc (size : usize , align : usize) -> * mut u8 { let layout_res = Layout :: from_size_align (size , align) ; if layout_res . is_err () || size == 0 { warn ! ("__sys_malloc called with size {size:#x}, align {align:#x} is an invalid layout!") ; return core :: ptr :: null_mut () ; } let layout = layout_res . unwrap () ; let ptr = unsafe { ALLOCATOR . alloc (layout) } ; trace ! ("__sys_malloc: allocate memory at {ptr:p} (size {size:#x}, align {align:#x})") ; ptr }
};
}
