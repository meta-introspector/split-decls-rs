// Generated macro for i16_sub_unsigned (function)
macro_rules! Depcrate_opsi16_sub_unsigned {
() => {
// Module: crate::ops
// Provides: {"i16_sub_unsigned"}
// Dependencies: {}
# [doc = " Computes `a - b` where `a` is signed and `b` is unsigned."] # [doc = ""] # [doc = " If overflow occurs, panics in debug mode and wraps in release mode."] # [inline (always)] pub fn i16_sub_unsigned (a : i16 , b : u16) -> i16 { let c = a . wrapping_sub (b as i16) ; debug_assert_eq ! (a as i32 - b as i32 , c as i32) ; c }
};
}
