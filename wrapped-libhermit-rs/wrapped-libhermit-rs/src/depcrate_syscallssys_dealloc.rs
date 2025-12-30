// Generated macro for sys_dealloc (function)
macro_rules! Depcrate_syscallssys_dealloc {
() => {
// Module: crate::syscalls
// Provides: {"sys_dealloc"}
// Dependencies: {}
# [doc = " Interface to deallocate a memory region from the system heap"] # [doc = ""] # [doc = " # Safety"] # [doc = " This function is unsafe because undefined behavior can result if the caller does not ensure all of the following:"] # [doc = " - ptr must denote a block of memory currently allocated via this allocator,"] # [doc = " - `size` and `align` must be the same values that were used to allocate that block of memory"] # [doc = " ToDO: verify if the same values for size and align always lead to the same layout"] # [doc = ""] # [doc = " # Errors"] # [doc = " May panic if debug assertions are enabled and invalid parameters `size` or `align` where passed."] # [cfg (all (target_os = "none" , not (feature = "common-os")))] # [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_dealloc (ptr : * mut u8 , size : usize , align : usize) { unsafe { let layout_res = Layout :: from_size_align (size , align) ; if layout_res . is_err () || size == 0 { warn ! ("__sys_dealloc called with size {size:#x}, align {align:#x} is an invalid layout!") ; debug_assert ! (layout_res . is_err () , "__sys_dealloc error: Invalid layout") ; debug_assert_ne ! (size , 0 , "__sys_dealloc error: size cannot be 0") ; } else { trace ! ("sys_free: deallocate memory at {ptr:p} (size {size:#x})") ; } let layout = layout_res . unwrap () ; ALLOCATOR . dealloc (ptr , layout) ; } }
};
}
