// Generated macro for macro_7491 (macro)
macro_rules! Depcrate_missing_const_for_thread_localmacro_7491 {
() => {
// Module: crate::missing_const_for_thread_local
// Provides: {"macro_7491"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Suggests to use `const` in `thread_local!` macro if possible."] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " The `thread_local!` macro wraps static declarations and makes them thread-local."] # [doc = " It supports using a `const` keyword that may be used for declarations that can"] # [doc = " be evaluated as a constant expression. This can enable a more efficient thread"] # [doc = " local implementation that can avoid lazy initialization. For types that do not"] # [doc = " need to be dropped, this can enable an even more efficient implementation that"] # [doc = " does not need to track any additional state."] # [doc = ""] # [doc = " https://doc.rust-lang.org/std/macro.thread_local.html"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " thread_local! {"] # [doc = "     static BUF: String = String::new();"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " thread_local! {"] # [doc = "     static BUF: String = const { String::new() };"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.77.0"] pub MISSING_CONST_FOR_THREAD_LOCAL , perf , "suggest using `const` in `thread_local!` macro" }
};
}
