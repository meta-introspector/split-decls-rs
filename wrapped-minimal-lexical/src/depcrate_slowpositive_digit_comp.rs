// Generated macro for positive_digit_comp (function)
macro_rules! Depcrate_slowpositive_digit_comp {
() => {
// Module: crate::slow
// Provides: {"positive_digit_comp"}
// Dependencies: {}
# [doc = " Generate the significant digits with a positive exponent relative to mantissa."] pub fn positive_digit_comp < F : Float > (mut bigmant : Bigint , exponent : i32) -> ExtendedFloat { bigmant . pow (10 , exponent as u32) . unwrap () ; let (mant , is_truncated) = bigmant . hi64 () ; let exp = bigmant . bit_length () as i32 - 64 + F :: EXPONENT_BIAS ; let mut fp = ExtendedFloat { mant , exp , } ; round :: < F , _ > (& mut fp , | f , s | { round_nearest_tie_even (f , s , | is_odd , is_halfway , is_above | { is_above || (is_halfway && is_truncated) || (is_odd && is_halfway) }) ; }) ; fp }
};
}
