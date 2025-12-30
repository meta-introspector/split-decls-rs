// Generated macro for macro_247 (macro)
macro_rules! Depcrate_assertions_on_result_statesmacro_247 {
() => {
// Module: crate::assertions_on_result_states
// Provides: {"macro_247"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `assert!(r.is_ok())` or `assert!(r.is_err())` calls."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " This form of assertion does not show any of the information present in the `Result`"] # [doc = " other than which variant it isn’t."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The suggested replacement decreases the readability of code and log output."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,no_run"] # [doc = " # let r = Ok::<_, ()>(());"] # [doc = " assert!(r.is_ok());"] # [doc = " # let r = Err::<(), _>(());"] # [doc = " assert!(r.is_err());"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # let r = Ok::<_, ()>(());"] # [doc = " r.unwrap();"] # [doc = " # let r = Err::<(), _>(());"] # [doc = " r.unwrap_err();"] # [doc = " ```"] # [clippy :: version = "1.64.0"] pub ASSERTIONS_ON_RESULT_STATES , restriction , "`assert!(r.is_ok())` or `assert!(r.is_err())` gives worse panic messages than directly calling `r.unwrap()` or `r.unwrap_err()`" }
};
}
