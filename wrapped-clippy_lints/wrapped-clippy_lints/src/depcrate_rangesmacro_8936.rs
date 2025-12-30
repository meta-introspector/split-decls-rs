// Generated macro for macro_8936 (macro)
macro_rules! Depcrate_rangesmacro_8936 {
() => {
// Module: crate::ranges
// Provides: {"macro_8936"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for expressions like `x >= 3 && x < 8` that could"] # [doc = " be more readably expressed as `(3..8).contains(x)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `contains` expresses the intent better and has less"] # [doc = " failure modes (such as fencepost errors or using `||` instead of `&&`)."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " // given"] # [doc = " let x = 6;"] # [doc = ""] # [doc = " assert!(x >= 3 && x < 8);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = "# let x = 6;"] # [doc = " assert!((3..8).contains(&x));"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub MANUAL_RANGE_CONTAINS , style , "manually reimplementing {`Range`, `RangeInclusive`}`::contains`" }
};
}
