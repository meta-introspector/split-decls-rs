// Generated macro for macro_7038 (macro)
macro_rules! Depcrate_methodsmacro_7038 {
() => {
// Module: crate::methods
// Provides: {"macro_7038"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " It checks for `str::bytes().count()` and suggests replacing it with"] # [doc = " `str::len()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `str::bytes().count()` is longer and may not be as performant as using"] # [doc = " `str::len()`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " \"hello\".bytes().count();"] # [doc = " String::from(\"hello\").bytes().count();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " \"hello\".len();"] # [doc = " String::from(\"hello\").len();"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub BYTES_COUNT_TO_LEN , complexity , "Using `bytes().count()` when `len()` performs the same functionality" }
};
}
