// Generated macro for macro_9986 (macro)
macro_rules! Depcrate_stringsmacro_9986 {
() => {
// Module: crate::strings
// Provides: {"macro_9986"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns about calling `str::trim` (or variants) before `str::split_whitespace`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `split_whitespace` already ignores leading and trailing whitespace."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " \" A B C \".trim().split_whitespace();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " \" A B C \".split_whitespace();"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub TRIM_SPLIT_WHITESPACE , style , "using `str::trim()` or alike before `str::split_whitespace`" }
};
}
