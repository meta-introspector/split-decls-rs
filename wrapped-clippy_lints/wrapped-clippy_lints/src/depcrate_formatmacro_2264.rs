// Generated macro for macro_2264 (macro)
macro_rules! Depcrate_formatmacro_2264 {
() => {
// Module: crate::format
// Provides: {"macro_2264"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the use of `format!(\"string literal with no"] # [doc = " argument\")` and `format!(\"{}\", foo)` where `foo` is a string."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " There is no point of doing that. `format!(\"foo\")` can"] # [doc = " be replaced by `\"foo\".to_owned()` if you really need a `String`. The even"] # [doc = " worse `&format!(\"foo\")` is often encountered in the wild. `format!(\"{}\","] # [doc = " foo)` can be replaced by `foo.clone()` if `foo: String` or `foo.to_owned()`"] # [doc = " if `foo: &str`."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = " let foo = \"foo\";"] # [doc = " format!(\"{}\", foo);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let foo = \"foo\";"] # [doc = " foo.to_owned();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub USELESS_FORMAT , complexity , "useless use of `format!`" }
};
}
