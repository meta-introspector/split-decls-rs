// Generated macro for into_float (function)
macro_rules! Depcrate_lexical_floatinto_float {
() => {
// Module: crate::lexical::float
// Provides: {"into_float"}
// Dependencies: {}
# [inline] pub (crate) fn into_float < F > (fp : ExtendedFloat) -> F where F : Float , { if fp . mant == 0 || fp . exp < F :: DENORMAL_EXPONENT { F :: ZERO } else if fp . exp >= F :: MAX_EXPONENT { F :: from_bits (F :: INFINITY_BITS) } else { let exp : u64 ; if (fp . exp == F :: DENORMAL_EXPONENT) && (fp . mant & F :: HIDDEN_BIT_MASK . as_u64 ()) == 0 { exp = 0 ; } else { exp = (fp . exp + F :: EXPONENT_BIAS) as u64 ; } let exp = exp << F :: MANTISSA_SIZE ; let mant = fp . mant & F :: MANTISSA_MASK . as_u64 () ; F :: from_bits (F :: Unsigned :: as_cast (mant | exp)) } }
};
}
