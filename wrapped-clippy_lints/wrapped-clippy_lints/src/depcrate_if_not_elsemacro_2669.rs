// Generated macro for macro_2669 (macro)
macro_rules! Depcrate_if_not_elsemacro_2669 {
() => {
// Module: crate::if_not_else
// Provides: {"macro_2669"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `!` or `!=` in an if condition with an"] # [doc = " else branch."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Negations reduce the readability of statements."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let v: Vec<usize> = vec![];"] # [doc = " # fn a() {}"] # [doc = " # fn b() {}"] # [doc = " if !v.is_empty() {"] # [doc = "     a()"] # [doc = " } else {"] # [doc = "     b()"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Could be written:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let v: Vec<usize> = vec![];"] # [doc = " # fn a() {}"] # [doc = " # fn b() {}"] # [doc = " if v.is_empty() {"] # [doc = "     b()"] # [doc = " } else {"] # [doc = "     a()"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub IF_NOT_ELSE , pedantic , "`if` branches that could be swapped so no negation operation is necessary on the condition" }
};
}
