// Generated macro for toint_2 (function)
macro_rules! Depcrate_datetoint_2 {
() => {
// Module: crate::date
// Provides: {"toint_2"}
// Dependencies: {}
fn toint_2 (s : & [u8]) -> Result < u8 , Error > { let high = s [0] . wrapping_sub (b'0') ; let low = s [1] . wrapping_sub (b'0') ; if high < 10 && low < 10 { Ok (high * 10 + low) } else { Err (Error (())) } }
};
}
