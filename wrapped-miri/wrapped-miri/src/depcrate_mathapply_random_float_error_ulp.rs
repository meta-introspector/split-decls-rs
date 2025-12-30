// Generated macro for apply_random_float_error_ulp (function)
macro_rules! Depcrate_mathapply_random_float_error_ulp {
() => {
// Module: crate::math
// Provides: {"apply_random_float_error_ulp"}
// Dependencies: {}
# [doc = " Applies an error of `[-N, +N]` ULP to the given value."] pub (crate) fn apply_random_float_error_ulp < F : rustc_apfloat :: Float > (ecx : & mut crate :: MiriInterpCx < '_ > , val : F , max_error : u32 ,) -> F { if ! ecx . machine . float_nondet || matches ! (ecx . machine . float_rounding_error , FloatRoundingErrorMode :: None) || val . is_zero () || ! val . is_finite () { return val ; } let rng = ecx . machine . rng . get_mut () ; let max_error = i64 :: from (max_error) ; let error = match ecx . machine . float_rounding_error { FloatRoundingErrorMode :: Random => rng . random_range (- max_error ..= max_error) , FloatRoundingErrorMode :: Max => if rng . random () { max_error } else { - max_error } , FloatRoundingErrorMode :: None => unreachable ! () , } ; let ulp = (((val . next_up () . value - val) . value + (val - val . next_down () . value) . value) . value / F :: from_u128 (2) . value) . value ; (val + (ulp * F :: from_i128 (error . into ()) . value) . value) . value }
};
}
