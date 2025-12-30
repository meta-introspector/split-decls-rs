// Generated macro for macro_10923 (macro)
macro_rules! Depcrate_useless_concatmacro_10923 {
() => {
// Module: crate::useless_concat
// Provides: {"macro_10923"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks that the `concat!` macro has at least two arguments."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " If there are less than 2 arguments, then calling the macro is doing nothing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = concat!(\"a\");"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = \"a\";"] # [doc = " ```"] # [clippy :: version = "1.89.0"] pub USELESS_CONCAT , complexity , "checks that the `concat` macro has at least two arguments" }
};
}
