// Generated macro for macro_4461 (macro)
macro_rules! Depcrate_manual_string_newmacro_4461 {
() => {
// Module: crate::manual_string_new
// Provides: {"macro_4461"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for usage of `\"\"` to create a `String`, such as `\"\".to_string()`, `\"\".to_owned()`,"] # [doc = " `String::from(\"\")` and others."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Different ways of creating an empty string makes your code less standardized, which can"] # [doc = " be confusing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let a = \"\".to_string();"] # [doc = " let b: String = \"\".into();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let a = String::new();"] # [doc = " let b = String::new();"] # [doc = " ```"] # [clippy :: version = "1.65.0"] pub MANUAL_STRING_NEW , pedantic , "empty String is being created manually" }
};
}
