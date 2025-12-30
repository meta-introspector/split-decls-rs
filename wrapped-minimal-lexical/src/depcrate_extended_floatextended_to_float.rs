// Generated macro for extended_to_float (function)
macro_rules! Depcrate_extended_floatextended_to_float {
() => {
// Module: crate::extended_float
// Provides: {"extended_to_float"}
// Dependencies: {}
# [doc = " Converts an `ExtendedFloat` to the closest machine float type."] # [inline (always)] pub fn extended_to_float < F : Float > (x : ExtendedFloat) -> F { let mut word = x . mant ; word |= (x . exp as u64) << F :: MANTISSA_SIZE ; F :: from_bits (word) }
};
}
