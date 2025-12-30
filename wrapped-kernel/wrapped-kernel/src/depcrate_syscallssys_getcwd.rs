// Generated macro for sys_getcwd (function)
macro_rules! Depcrate_syscallssys_getcwd {
() => {
// Module: crate::syscalls
// Provides: {"sys_getcwd"}
// Dependencies: {}
# [hermit_macro :: system] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_getcwd (buf : * mut c_char , size : usize) -> * const c_char { let error = | e : Errno | { e . set_errno () ; null :: < c_char > () } ; if size == 0 { return error (Errno :: Inval) ; } if buf . is_null () { return error (Errno :: Noent) ; } let cwd = match fs :: get_cwd () { Err (e) => { return error (e) ; } Ok (cwd) => cwd , } ; let Ok (cwd) = CString :: new (cwd) else { return error (Errno :: Noent) ; } ; if (cwd . count_bytes () + 1) > size { return error (Errno :: Range) ; } unsafe { buf . copy_from (cwd . as_ptr () , size) ; } buf }
};
}
