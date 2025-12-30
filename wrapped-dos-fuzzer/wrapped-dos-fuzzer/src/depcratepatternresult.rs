// Generated macro for PatternResult (enum)
macro_rules! DepcratePatternResult {
() => {
// Module: crate
// Provides: {"PatternResult"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] enum PatternResult { # [doc = " Probably linear growth (score)"] Linear (f64) , # [doc = " Probably non-linear growth (score)"] NonLinear (f64) , # [doc = " Execution took too long, assumed non-linear"] TooLong , }
};
}
