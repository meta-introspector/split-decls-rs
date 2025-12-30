// Generated macro for wrapper_flags_to_string (function)
macro_rules! Depcrate_output_render_flags_bsdwrapper_flags_to_string {
() => {
// Module: crate::output::render::flags_bsd
// Provides: {"wrapper_flags_to_string"}
// Dependencies: {}
# [doc = " Wrapper around the C library call fflagstostr or the netbsd equivalent"] # [doc = " If returned string is NULL or empty a \"-\" is returned"] fn wrapper_flags_to_string (flags : f :: flag_t) -> String { # [cfg (target_os = "netbsd")] let empty_string = CString :: new ("") . expect ("This string is always valid") ; # [cfg (not (target_os = "netbsd"))] let flags_c_str = unsafe { fflagstostr (libc :: c_ulong :: from (flags)) } ; # [cfg (target_os = "netbsd")] let flags_c_str = unsafe { flags_to_string (libc :: c_ulong :: from (flags) , empty_string . as_ptr ()) } ; if flags_c_str . is_null () { "-" . to_string () } else { let flags_str = unsafe { CStr :: from_ptr (flags_c_str) } ; let flags = flags_str . to_str () . map_or ("-" , | s | if s . is_empty () { "-" } else { s }) . to_string () ; unsafe { libc :: free (flags_c_str . cast_mut () . cast ()) ; } flags } }
};
}
