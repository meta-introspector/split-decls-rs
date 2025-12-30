// Generated macro for mantissa_exponent (function)
macro_rules! Depcrate_lexical_exponentmantissa_exponent {
() => {
// Module: crate::lexical::exponent
// Provides: {"mantissa_exponent"}
// Dependencies: {}
# [inline] pub (crate) fn mantissa_exponent (exponent : i32 , fraction_digits : usize , truncated : usize) -> i32 { if fraction_digits > truncated { exponent . saturating_sub (into_i32 (fraction_digits - truncated)) } else { exponent . saturating_add (into_i32 (truncated - fraction_digits)) } }
};
}
