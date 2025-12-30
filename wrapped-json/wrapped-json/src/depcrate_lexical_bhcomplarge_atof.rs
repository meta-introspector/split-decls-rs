// Generated macro for large_atof (function)
macro_rules! Depcrate_lexical_bhcomplarge_atof {
() => {
// Module: crate::lexical::bhcomp
// Provides: {"large_atof"}
// Dependencies: {}
# [doc = " Calculate the mantissa for a big integer with a positive exponent."] fn large_atof < F > (mantissa : Bigint , exponent : i32) -> F where F : Float , { let bits = mem :: size_of :: < u64 > () * 8 ; let mut bigmant = mantissa ; bigmant . imul_pow10 (exponent as u32) ; let (mant , is_truncated) = bigmant . hi64 () ; let exp = bigmant . bit_length () as i32 - bits as i32 ; let mut fp = ExtendedFloat { mant , exp } ; fp . round_to_native :: < F , _ > (| fp , shift | round_nearest_tie_even (fp , shift , is_truncated)) ; into_float (fp) }
};
}
