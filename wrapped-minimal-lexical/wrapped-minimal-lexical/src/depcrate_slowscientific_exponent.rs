// Generated macro for scientific_exponent (function)
macro_rules! Depcrate_slowscientific_exponent {
() => {
// Module: crate::slow
// Provides: {"scientific_exponent"}
// Dependencies: {}
# [doc = " Calculate the scientific exponent from a `Number` value."] # [doc = " Any other attempts would require slowdowns for faster algorithms."] # [inline] pub fn scientific_exponent (num : & Number) -> i32 { let mut mantissa = num . mantissa ; let mut exponent = num . exponent ; while mantissa >= 10000 { mantissa /= 10000 ; exponent += 4 ; } while mantissa >= 100 { mantissa /= 100 ; exponent += 2 ; } while mantissa >= 10 { mantissa /= 10 ; exponent += 1 ; } exponent as i32 }
};
}
