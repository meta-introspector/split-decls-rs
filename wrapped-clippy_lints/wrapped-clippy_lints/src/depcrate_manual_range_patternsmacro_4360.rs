// Generated macro for macro_4360 (macro)
macro_rules! Depcrate_manual_range_patternsmacro_4360 {
() => {
// Module: crate::manual_range_patterns
// Provides: {"macro_4360"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Looks for combined OR patterns that are all contained in a specific range,"] # [doc = " e.g. `6 | 4 | 5 | 9 | 7 | 8` can be rewritten as `4..=9`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using an explicit range is more concise and easier to read."] # [doc = ""] # [doc = " ### Known issues"] # [doc = " This lint intentionally does not handle numbers greater than `i128::MAX` for `u128` literals"] # [doc = " in order to support negative numbers."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = 6;"] # [doc = " let foo = matches!(x, 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = 6;"] # [doc = " let foo = matches!(x, 1..=10);"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub MANUAL_RANGE_PATTERNS , complexity , "manually writing range patterns using a combined OR pattern (`|`)" }
};
}
