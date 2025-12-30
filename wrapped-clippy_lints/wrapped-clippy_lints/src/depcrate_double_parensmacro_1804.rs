// Generated macro for macro_1804 (macro)
macro_rules! Depcrate_double_parensmacro_1804 {
() => {
// Module: crate::double_parens
// Provides: {"macro_1804"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for unnecessary double parentheses."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This makes code harder to read and might indicate a"] # [doc = " mistake."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn simple_double_parens() -> i32 {"] # [doc = "     ((0))"] # [doc = " }"] # [doc = ""] # [doc = " # fn foo(bar: usize) {}"] # [doc = " foo((0));"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn simple_no_parens() -> i32 {"] # [doc = "     (0)"] # [doc = " }"] # [doc = ""] # [doc = " # fn foo(bar: usize) {}"] # [doc = " foo(0);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub DOUBLE_PARENS , complexity , "Warn on unnecessary double parentheses" }
};
}
