// Generated macro for from_float (function)
macro_rules! Depcrate_lexical_floatfrom_float {
() => {
// Module: crate::lexical::float
// Provides: {"from_float"}
// Dependencies: {}
# [inline] pub (crate) fn from_float < F > (f : F) -> ExtendedFloat where F : Float , { ExtendedFloat { mant : u64 :: as_cast (f . mantissa ()) , exp : f . exponent () , } }
};
}
