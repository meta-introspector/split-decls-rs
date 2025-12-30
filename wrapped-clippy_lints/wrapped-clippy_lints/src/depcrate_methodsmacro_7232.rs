// Generated macro for macro_7232 (macro)
macro_rules! Depcrate_methodsmacro_7232 {
() => {
// Module: crate::methods
// Provides: {"macro_7232"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.map_or_else()` \"map closure\" for `Result` type."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This can be written more concisely by using `unwrap_or_else()`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # fn handle_error(_: ()) -> u32 { 0 }"] # [doc = " let x: Result<u32, ()> = Ok(0);"] # [doc = " let y = x.map_or_else(|err| handle_error(err), |n| n);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # fn handle_error(_: ()) -> u32 { 0 }"] # [doc = " let x: Result<u32, ()> = Ok(0);"] # [doc = " let y = x.unwrap_or_else(|err| handle_error(err));"] # [doc = " ```"] # [clippy :: version = "1.78.0"] pub UNNECESSARY_RESULT_MAP_OR_ELSE , suspicious , "making no use of the \"map closure\" when calling `.map_or_else(|err| handle_error(err), |n| n)`" }
};
}
