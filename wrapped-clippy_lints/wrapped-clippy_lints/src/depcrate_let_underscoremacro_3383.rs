// Generated macro for macro_3383 (macro)
macro_rules! Depcrate_let_underscoremacro_3383 {
() => {
// Module: crate::let_underscore
// Provides: {"macro_3383"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `let _ = <expr>` where expr is `#[must_use]`"] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " To ensure that all `#[must_use]` types are used rather than ignored."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn f() -> Result<u32, u32> {"] # [doc = "     Ok(0)"] # [doc = " }"] # [doc = ""] # [doc = " let _ = f();"] # [doc = " // is_ok() is marked #[must_use]"] # [doc = " let _ = f().is_ok();"] # [doc = " ```"] # [clippy :: version = "1.42.0"] pub LET_UNDERSCORE_MUST_USE , restriction , "non-binding `let` on a `#[must_use]` expression" }
};
}
