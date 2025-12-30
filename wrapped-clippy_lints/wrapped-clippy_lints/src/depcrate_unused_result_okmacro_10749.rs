// Generated macro for macro_10749 (macro)
macro_rules! Depcrate_unused_result_okmacro_10749 {
() => {
// Module: crate::unused_result_ok
// Provides: {"macro_10749"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `Result::ok()` without using the returned `Option`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `Result::ok()` may look like the result is checked like `unwrap` or `expect` would do"] # [doc = " but it only silences the warning caused by `#[must_use]` on the `Result`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # fn some_function() -> Result<(), ()> { Ok(()) }"] # [doc = " some_function().ok();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # fn some_function() -> Result<(), ()> { Ok(()) }"] # [doc = " let _ = some_function();"] # [doc = " ```"] # [clippy :: version = "1.82.0"] pub UNUSED_RESULT_OK , restriction , "Use of `.ok()` to silence `Result`'s `#[must_use]` is misleading. Use `let _ =` instead." }
};
}
