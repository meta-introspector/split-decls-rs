// Generated macro for f64p1_with_underscores (function)
macro_rules! Depcrate_thousandsf64p1_with_underscores {
() => {
// Module: crate::thousands
// Provides: {"f64p1_with_underscores"}
// Dependencies: {}
# [doc = " Print an `f64` with precision 1 (one decimal place) and underscore separators."] pub fn f64p1_with_underscores (n : f64) -> String { format_with_underscores (format ! ("{n:.1}")) }
};
}
