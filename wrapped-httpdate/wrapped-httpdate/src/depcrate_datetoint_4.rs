// Generated macro for toint_4 (function)
macro_rules! Depcrate_datetoint_4 {
() => {
// Module: crate::date
// Provides: {"toint_4"}
// Dependencies: {}
# [allow (clippy :: many_single_char_names)] fn toint_4 (s : & [u8]) -> Result < u16 , Error > { let a = u16 :: from (s [0] . wrapping_sub (b'0')) ; let b = u16 :: from (s [1] . wrapping_sub (b'0')) ; let c = u16 :: from (s [2] . wrapping_sub (b'0')) ; let d = u16 :: from (s [3] . wrapping_sub (b'0')) ; if a < 10 && b < 10 && c < 10 && d < 10 { Ok (a * 1000 + b * 100 + c * 10 + d) } else { Err (Error (())) } }
};
}
