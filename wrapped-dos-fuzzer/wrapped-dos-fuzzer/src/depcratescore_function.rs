// Generated macro for SCORE_FUNCTION (const)
macro_rules! DepcrateSCORE_FUNCTION {
() => {
// Module: crate
// Provides: {"SCORE_FUNCTION"}
// Dependencies: {}
# [doc = " Score function to use when figuring out if something is non-linear."] # [doc = ""] # [doc = " Possible values: `scoring::{slope_stddev,pearson_correlation}`"] const SCORE_FUNCTION : fn (& [(f64 , f64)]) -> (f64 , bool) = scoring :: slope_stddev ;
};
}
