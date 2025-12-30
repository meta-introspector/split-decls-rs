// Generated macro for macro_7249 (macro)
macro_rules! Depcrate_methodsmacro_7249 {
() => {
// Module: crate::methods
// Provides: {"macro_7249"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `iter().any()` on slices when it can be replaced with `contains()` and suggests doing so."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `contains()` is more concise and idiomatic, while also being faster in some cases."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo(values: &[u8]) -> bool {"] # [doc = "     values.iter().any(|&v| v == 10)"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn foo(values: &[u8]) -> bool {"] # [doc = "     values.contains(&10)"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.87.0"] pub MANUAL_CONTAINS , perf , "unnecessary `iter().any()` on slices that can be replaced with `contains()`" }
};
}
