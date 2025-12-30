// Generated macro for weighted (function)
macro_rules! Depcrate_boolweighted {
() => {
// Module: crate::bool
// Provides: {"weighted"}
// Dependencies: {}
# [doc = " Generates boolean values by picking `true` with the given `probability`"] # [doc = " (1.0 = always true, 0.0 = always false)."] # [doc = ""] # [doc = " Shrinks `true` to `false`."] pub fn weighted (probability : f64) -> Weighted { Weighted (probability) }
};
}
