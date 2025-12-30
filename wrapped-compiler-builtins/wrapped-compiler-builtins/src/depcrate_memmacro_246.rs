// Generated macro for macro_246 (macro)
macro_rules! Depcrate_memmacro_246 {
() => {
// Module: crate::mem
// Provides: {"macro_246"}
// Dependencies: {}
intrinsics ! { # [mem_builtin] pub unsafe extern "C" fn memcpy (dest : * mut u8 , src : * const u8 , n : usize) -> * mut u8 { impls :: copy_forward (dest , src , n) ; dest } # [mem_builtin] pub unsafe extern "C" fn memmove (dest : * mut u8 , src : * const u8 , n : usize) -> * mut u8 { let delta = (dest as usize) . wrapping_sub (src as usize) ; if delta >= n { impls :: copy_forward (dest , src , n) ; } else { impls :: copy_backward (dest , src , n) ; } dest } # [mem_builtin] pub unsafe extern "C" fn memset (s : * mut u8 , c : core :: ffi :: c_int , n : usize) -> * mut u8 { impls :: set_bytes (s , c as u8 , n) ; s } # [mem_builtin] pub unsafe extern "C" fn memcmp (s1 : * const u8 , s2 : * const u8 , n : usize) -> core :: ffi :: c_int { impls :: compare_bytes (s1 , s2 , n) } # [mem_builtin] pub unsafe extern "C" fn bcmp (s1 : * const u8 , s2 : * const u8 , n : usize) -> core :: ffi :: c_int { memcmp (s1 , s2 , n) } # [mem_builtin] pub unsafe extern "C" fn strlen (s : * const core :: ffi :: c_char) -> usize { impls :: c_string_length (s) } }
};
}
