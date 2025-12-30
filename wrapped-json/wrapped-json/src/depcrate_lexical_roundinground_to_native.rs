// Generated macro for round_to_native (function)
macro_rules! Depcrate_lexical_roundinground_to_native {
() => {
// Module: crate::lexical::rounding
// Provides: {"round_to_native"}
// Dependencies: {}
# [inline] pub (crate) fn round_to_native < F , Algorithm > (fp : & mut ExtendedFloat , algorithm : Algorithm) where F : Float , Algorithm : FnOnce (& mut ExtendedFloat , i32) , { fp . normalize () ; round_to_float :: < F , _ > (fp , algorithm) ; avoid_overflow :: < F > (fp) ; }
};
}
