// Generated macro for i16_abs_sub (function)
macro_rules! Depcrate_opsi16_abs_sub {
() => {
// Module: crate::ops
// Provides: {"i16_abs_sub"}
// Dependencies: {}
# [doc = " Computes `a - b` where `a >= b`."] # [doc = ""] # [doc = " Overflow cannot occur because the result is unsigned."] # [doc = ""] # [doc = " This is similar to `abs_diff` but with the additional constraint that `a >= b`."] # [doc = ""] # [doc = " If `a < b`, panics in debug mode and wraps in release mode."] # [inline (always)] pub fn i16_abs_sub (a : i16 , b : i16) -> u16 { debug_assert ! (a >= b) ; let c = (a as u16) . wrapping_sub (b as u16) ; debug_assert_eq ! (a as i32 - b as i32 , c as i32) ; c }
};
}
