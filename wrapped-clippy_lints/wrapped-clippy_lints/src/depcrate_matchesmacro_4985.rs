// Generated macro for macro_4985 (macro)
macro_rules! Depcrate_matchesmacro_4985 {
() => {
// Module: crate::matches
// Provides: {"macro_4985"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Finds patterns that reimplement `Option::unwrap_or` or `Result::unwrap_or`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Concise code helps focusing on behavior instead of boilerplate."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let foo: Option<i32> = None;"] # [doc = " match foo {"] # [doc = "     Some(v) => v,"] # [doc = "     None => 1,"] # [doc = " };"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let foo: Option<i32> = None;"] # [doc = " foo.unwrap_or(1);"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub MANUAL_UNWRAP_OR , complexity , "finds patterns that can be encoded more concisely with `Option::unwrap_or` or `Result::unwrap_or`" }
};
}
