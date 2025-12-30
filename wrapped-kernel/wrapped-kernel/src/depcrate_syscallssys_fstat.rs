// Generated macro for sys_fstat (function)
macro_rules! Depcrate_syscallssys_fstat {
() => {
// Module: crate::syscalls
// Provides: {"sys_fstat"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_fstat (fd : FileDescriptor , stat : * mut FileAttr) -> i32 { if stat . is_null () { return - i32 :: from (Errno :: Inval) ; } crate :: fd :: fstat (fd) . map_or_else (| e | - i32 :: from (e) , | v | unsafe { * stat = v ; 0 } ,) }
};
}
