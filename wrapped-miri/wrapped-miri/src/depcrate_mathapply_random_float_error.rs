// Generated macro for apply_random_float_error (function)
macro_rules! Depcrate_mathapply_random_float_error {
() => {
// Module: crate::math
// Provides: {"apply_random_float_error"}
// Dependencies: {}
# [doc = " Disturbes a floating-point result by a relative error in the range (-2^scale, 2^scale)."] pub (crate) fn apply_random_float_error < F : rustc_apfloat :: Float > (ecx : & mut crate :: MiriInterpCx < '_ > , val : F , err_scale : i32 ,) -> F { if ! ecx . machine . float_nondet || matches ! (ecx . machine . float_rounding_error , FloatRoundingErrorMode :: None) || val . is_zero () || ! val . is_finite () { return val ; } let rng = ecx . machine . rng . get_mut () ; let r = F :: from_u128 (match ecx . machine . float_rounding_error { FloatRoundingErrorMode :: Random => rng . random_range (0 .. (1 << F :: PRECISION)) , FloatRoundingErrorMode :: Max => (1 << F :: PRECISION) - 1 , FloatRoundingErrorMode :: None => unreachable ! () , }) . value ; let err = r . scalbn (err_scale . strict_sub (F :: PRECISION . try_into () . unwrap ())) ; let err = if rng . random () { - err } else { err } ; (val + (val * err) . value) . value }
};
}
