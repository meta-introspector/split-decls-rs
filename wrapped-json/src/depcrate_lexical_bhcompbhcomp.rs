// Generated macro for bhcomp (function)
macro_rules! Depcrate_lexical_bhcompbhcomp {
() => {
// Module: crate::lexical::bhcomp
// Provides: {"bhcomp"}
// Dependencies: {}
# [doc = " Calculate the exact value of the float."] # [doc = ""] # [doc = " Note: fraction must not have trailing zeros."] pub (crate) fn bhcomp < F > (b : F , integer : & [u8] , mut fraction : & [u8] , exponent : i32) -> F where F : Float , { let integer_digits = integer . len () ; let fraction_digits = fraction . len () ; let digits_start = if integer_digits == 0 { let start = fraction . iter () . take_while (| & x | * x == b'0') . count () ; fraction = & fraction [start ..] ; start } else { 0 } ; let sci_exp = scientific_exponent (exponent , integer_digits , digits_start) ; let count = F :: MAX_DIGITS . min (integer_digits + fraction_digits - digits_start) ; let scaled_exponent = sci_exp + 1 - count as i32 ; let mantissa = parse_mantissa :: < F > (integer , fraction) ; if scaled_exponent >= 0 { large_atof (mantissa , scaled_exponent) } else { small_atof (mantissa , scaled_exponent , b) } }
};
}
