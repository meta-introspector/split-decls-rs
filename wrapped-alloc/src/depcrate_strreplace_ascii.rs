// Generated macro for replace_ascii (function)
macro_rules! Depcrate_strreplace_ascii {
() => {
// Module: crate::str
// Provides: {"replace_ascii"}
// Dependencies: {}
# [inline] # [cfg (not (no_global_oom_handling))] # [allow (dead_code)] # [doc = " Faster implementation of string replacement for ASCII to ASCII cases."] # [doc = " Should produce fast vectorized code."] unsafe fn replace_ascii (utf8_bytes : & [u8] , from : u8 , to : u8) -> String { let result : Vec < u8 > = utf8_bytes . iter () . map (| b | if * b == from { to } else { * b }) . collect () ; unsafe { String :: from_utf8_unchecked (result) } }
};
}
