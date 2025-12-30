// Generated macro for macro_10936 (macro)
macro_rules! Depcrate_unnecessary_semicolonmacro_10936 {
() => {
// Module: crate::unnecessary_semicolon
// Provides: {"macro_10936"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the presence of a semicolon at the end of"] # [doc = " a `match` or `if` statement evaluating to `()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The semicolon is not needed, and may be removed to"] # [doc = " avoid confusion and visual clutter."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let a: u32 = 42;"] # [doc = " if a > 10 {"] # [doc = "     println!(\"a is greater than 10\");"] # [doc = " };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let a: u32 = 42;"] # [doc = " if a > 10 {"] # [doc = "     println!(\"a is greater than 10\");"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.86.0"] pub UNNECESSARY_SEMICOLON , pedantic , "unnecessary semicolon after expression returning `()`" }
};
}
