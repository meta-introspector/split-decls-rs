// Generated macro for fallback_path (function)
macro_rules! Depcrate_lexical_algorithmfallback_path {
() => {
// Module: crate::lexical::algorithm
// Provides: {"fallback_path"}
// Dependencies: {}
# [doc = " Fallback path when the fast path does not work."] # [doc = ""] # [doc = " Uses the moderate path, if applicable, otherwise, uses the slow path"] # [doc = " as required."] pub (crate) fn fallback_path < F > (integer : & [u8] , fraction : & [u8] , mantissa : u64 , exponent : i32 , mantissa_exponent : i32 , truncated : bool ,) -> F where F : Float , { let (fp , valid) = moderate_path :: < F > (mantissa , mantissa_exponent , truncated) ; if valid { return fp . into_float :: < F > () ; } let b = fp . into_downward_float :: < F > () ; if b . is_special () { b } else { bhcomp (b , integer , fraction , exponent) } }
};
}
