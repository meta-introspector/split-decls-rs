// Generated macro for sys_alloc_zeroed (function)
macro_rules! Depcrate_syscallssys_alloc_zeroed {
() => {
// Module: crate::syscalls
// Provides: {"sys_alloc_zeroed"}
// Dependencies: {}
# [cfg (all (target_os = "none" , not (feature = "common-os")))] # [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_alloc_zeroed (size : usize , align : usize) -> * mut u8 { let layout_res = Layout :: from_size_align (size , align) ; if layout_res . is_err () || size == 0 { warn ! ("__sys_alloc_zeroed called with size {size:#x}, align {align:#x} is an invalid layout!") ; return core :: ptr :: null_mut () ; } let layout = layout_res . unwrap () ; let ptr = unsafe { ALLOCATOR . alloc_zeroed (layout) } ; trace ! ("__sys_alloc_zeroed: allocate memory at {ptr:p} (size {size:#x}, align {align:#x})") ; ptr }
};
}
