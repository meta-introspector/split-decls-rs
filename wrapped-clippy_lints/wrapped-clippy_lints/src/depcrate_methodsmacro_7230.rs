// Generated macro for macro_7230 (macro)
macro_rules! Depcrate_methodsmacro_7230 {
() => {
// Module: crate::methods
// Provides: {"macro_7230"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.as_ref().cloned()` and `.as_mut().cloned()` on `Option`s"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This can be written more concisely by cloning the `Option` directly."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo(bar: &Option<Vec<u8>>) -> Option<Vec<u8>> {"] # [doc = "     bar.as_ref().cloned()"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn foo(bar: &Option<Vec<u8>>) -> Option<Vec<u8>> {"] # [doc = "     bar.clone()"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.77.0"] pub OPTION_AS_REF_CLONED , pedantic , "cloning an `Option` via `as_ref().cloned()`" }
};
}
