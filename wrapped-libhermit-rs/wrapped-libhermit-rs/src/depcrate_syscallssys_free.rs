// Generated macro for sys_free (function)
macro_rules! Depcrate_syscallssys_free {
() => {
// Module: crate::syscalls
// Provides: {"sys_free"}
// Dependencies: {}
# [cfg (all (target_os = "none" , not (feature = "common-os")))] # [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_free (ptr : * mut u8 , size : usize , align : usize) { unsafe { let layout_res = Layout :: from_size_align (size , align) ; if layout_res . is_err () || size == 0 { warn ! ("__sys_free called with size {size:#x}, align {align:#x} is an invalid layout!") ; debug_assert ! (layout_res . is_err () , "__sys_free error: Invalid layout") ; debug_assert_ne ! (size , 0 , "__sys_free error: size cannot be 0") ; } else { trace ! ("sys_free: deallocate memory at {ptr:p} (size {size:#x})") ; } let layout = layout_res . unwrap () ; ALLOCATOR . dealloc (ptr , layout) ; } }
};
}
