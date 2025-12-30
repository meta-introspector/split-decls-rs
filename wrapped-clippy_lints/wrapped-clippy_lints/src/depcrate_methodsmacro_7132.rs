// Generated macro for macro_7132 (macro)
macro_rules! Depcrate_methodsmacro_7132 {
() => {
// Module: crate::methods
// Provides: {"macro_7132"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.clone()` on a `Copy` type."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The only reason `Copy` types implement `Clone` is for"] # [doc = " generics, not for using the `clone` method on a concrete type."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " 42u64.clone();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub CLONE_ON_COPY , complexity , "using `clone` on a `Copy` type" }
};
}
