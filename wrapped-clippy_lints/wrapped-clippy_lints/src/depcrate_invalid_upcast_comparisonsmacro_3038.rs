// Generated macro for macro_3038 (macro)
macro_rules! Depcrate_invalid_upcast_comparisonsmacro_3038 {
() => {
// Module: crate::invalid_upcast_comparisons
// Provides: {"macro_3038"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for comparisons where the relation is always either"] # [doc = " true or false, but where one side has been upcast so that the comparison is"] # [doc = " necessary. Only integer types are checked."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " An expression like `let x : u8 = ...; (x as u32) > 300`"] # [doc = " will mistakenly imply that it is possible for `x` to be outside the range of"] # [doc = " `u8`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x: u8 = 1;"] # [doc = " (x as u32) > 300;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub INVALID_UPCAST_COMPARISONS , pedantic , "a comparison involving an upcast which is always true or false" }
};
}
