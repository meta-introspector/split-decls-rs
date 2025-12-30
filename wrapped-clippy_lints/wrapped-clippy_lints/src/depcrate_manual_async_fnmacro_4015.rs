// Generated macro for macro_4015 (macro)
macro_rules! Depcrate_manual_async_fnmacro_4015 {
() => {
// Module: crate::manual_async_fn
// Provides: {"macro_4015"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " It checks for manual implementations of `async` functions."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's more idiomatic to use the dedicated syntax."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::future::Future;"] # [doc = ""] # [doc = " fn foo() -> impl Future<Output = i32> { async { 42 } }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " async fn foo() -> i32 { 42 }"] # [doc = " ```"] # [clippy :: version = "1.45.0"] pub MANUAL_ASYNC_FN , style , "manual implementations of `async` functions can be simplified using the dedicated syntax" }
};
}
