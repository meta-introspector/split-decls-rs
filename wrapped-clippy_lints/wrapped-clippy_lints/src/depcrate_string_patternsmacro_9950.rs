// Generated macro for macro_9950 (macro)
macro_rules! Depcrate_string_patternsmacro_9950 {
() => {
// Module: crate::string_patterns
// Provides: {"macro_9950"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for string methods that receive a single-character"] # [doc = " `str` as an argument, e.g., `_.split(\"x\")`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " While this can make a perf difference on some systems,"] # [doc = " benchmarks have proven inconclusive. But at least using a"] # [doc = " char literal makes it clear that we are looking at a single"] # [doc = " character."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Does not catch multi-byte unicode characters. This is by"] # [doc = " design, on many machines, splitting by a non-ascii char is"] # [doc = " actually slower. Please do your own measurements instead of"] # [doc = " relying solely on the results of this lint."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " _.split(\"x\");"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " _.split('x');"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SINGLE_CHAR_PATTERN , pedantic , "using a single-character str where a char could be used, e.g., `_.split(\"x\")`" }
};
}
