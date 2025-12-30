// Generated macro for is_non_zero (function)
macro_rules! Depcrate_portableis_non_zero {
() => {
// Module: crate::portable
// Provides: {"is_non_zero"}
// Dependencies: {}
# [doc = " Check if the given condition value is non-zero"] # [doc = ""] # [doc = " # Returns"] # [doc = " - `condition` is zero: `0`"] # [doc = " - `condition` is non-zero: `1`"] # [inline] fn is_non_zero (condition : Condition) -> u64 { const SHIFT_BITS : usize = size_of :: < u64 > () - 1 ; let condition = condition as u64 ; ((condition | (! condition) . wrapping_add (1)) >> SHIFT_BITS) & 1 }
};
}
