// Generated macro for moderate_path (function)
macro_rules! Depcrate_parsemoderate_path {
() => {
// Module: crate::parse
// Provides: {"moderate_path"}
// Dependencies: {}
# [doc = " Wrapper for different moderate-path algorithms."] # [doc = " A return exponent of `-1` indicates an invalid value."] # [inline] pub fn moderate_path < F : Float > (num : & Number) -> ExtendedFloat { # [cfg (not (feature = "compact"))] return lemire :: < F > (num) ; # [cfg (feature = "compact")] return bellerophon :: < F > (num) ; }
};
}
