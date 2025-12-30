// Generated macro for macro_5043 (macro)
macro_rules! Depcrate_matchesmacro_5043 {
() => {
// Module: crate::matches
// Provides: {"macro_5043"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for manual implementation of `.ok()` or `.err()`"] # [doc = " on `Result` values."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `.ok()` or `.err()` rather than a `match` or"] # [doc = " `if let` is less complex and more readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # fn func() -> Result<u32, &'static str> { Ok(0) }"] # [doc = " let a = match func() {"] # [doc = "     Ok(v) => Some(v),"] # [doc = "     Err(_) => None,"] # [doc = " };"] # [doc = " let b = if let Err(v) = func() {"] # [doc = "     Some(v)"] # [doc = " } else {"] # [doc = "     None"] # [doc = " };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # fn func() -> Result<u32, &'static str> { Ok(0) }"] # [doc = " let a = func().ok();"] # [doc = " let b = func().err();"] # [doc = " ```"] # [clippy :: version = "1.86.0"] pub MANUAL_OK_ERR , complexity , "find manual implementations of `.ok()` or `.err()` on `Result`" }
};
}
