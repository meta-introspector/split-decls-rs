// Generated macro for macro_6971 (macro)
macro_rules! Depcrate_methodsmacro_6971 {
() => {
// Module: crate::methods
// Provides: {"macro_6971"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_.map_or(None, Some)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more concisely as"] # [doc = " `_.ok()`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let r: Result<u32, &str> = Ok(1);"] # [doc = " assert_eq!(Some(1), r.map_or(None, Some));"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let r: Result<u32, &str> = Ok(1);"] # [doc = " assert_eq!(Some(1), r.ok());"] # [doc = " ```"] # [clippy :: version = "1.44.0"] pub RESULT_MAP_OR_INTO_OPTION , style , "using `Result.map_or(None, Some)`, which is more succinctly expressed as `ok()`" }
};
}
