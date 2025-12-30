// Generated macro for macro_4976 (macro)
macro_rules! Depcrate_matchesmacro_4976 {
() => {
// Module: crate::matches
// Provides: {"macro_4976"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for wildcard pattern used with others patterns in same match arm."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Wildcard pattern already covers any other pattern as it will match anyway."] # [doc = " It makes the code less readable, especially to spot wildcard pattern use in match arm."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let s = \"foo\";"] # [doc = " match s {"] # [doc = "     \"a\" => {},"] # [doc = "     \"bar\" | _ => {},"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let s = \"foo\";"] # [doc = " match s {"] # [doc = "     \"a\" => {},"] # [doc = "     _ => {},"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.42.0"] pub WILDCARD_IN_OR_PATTERNS , complexity , "a wildcard pattern used with others patterns in same match arm" }
};
}
