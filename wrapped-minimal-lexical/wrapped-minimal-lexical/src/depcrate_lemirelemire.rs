// Generated macro for lemire (function)
macro_rules! Depcrate_lemirelemire {
() => {
// Module: crate::lemire
// Provides: {"lemire"}
// Dependencies: {}
# [doc = " Ensure truncation of digits doesn't affect our computation, by doing 2 passes."] # [inline] pub fn lemire < F : Float > (num : & Number) -> ExtendedFloat { let mut fp = compute_float :: < F > (num . exponent , num . mantissa) ; if num . many_digits && fp . exp >= 0 && fp != compute_float :: < F > (num . exponent , num . mantissa + 1) { fp = compute_error :: < F > (num . exponent , num . mantissa) ; } fp }
};
}
