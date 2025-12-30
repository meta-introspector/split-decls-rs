// Generated macro for num_decimal_digits (function)
macro_rules! Depcrate_renderer_rendernum_decimal_digits {
() => {
// Module: crate::renderer::render
// Provides: {"num_decimal_digits"}
// Dependencies: {}
fn num_decimal_digits (num : usize) -> usize { # [cfg (target_pointer_width = "64")] const MAX_DIGITS : usize = 20 ; # [cfg (target_pointer_width = "32")] const MAX_DIGITS : usize = 10 ; # [cfg (target_pointer_width = "16")] const MAX_DIGITS : usize = 5 ; let mut lim = 10 ; for num_digits in 1 .. MAX_DIGITS { if num < lim { return num_digits ; } lim = lim . wrapping_mul (10) ; } MAX_DIGITS }
};
}
