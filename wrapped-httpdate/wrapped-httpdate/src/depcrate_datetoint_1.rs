// Generated macro for toint_1 (function)
macro_rules! Depcrate_datetoint_1 {
() => {
// Module: crate::date
// Provides: {"toint_1"}
// Dependencies: {}
fn toint_1 (x : u8) -> Result < u8 , Error > { let result = x . wrapping_sub (b'0') ; if result < 10 { Ok (result) } else { Err (Error (())) } }
};
}
