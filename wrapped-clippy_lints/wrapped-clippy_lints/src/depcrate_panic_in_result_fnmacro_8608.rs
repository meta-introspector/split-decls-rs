// Generated macro for macro_8608 (macro)
macro_rules! Depcrate_panic_in_result_fnmacro_8608 {
() => {
// Module: crate::panic_in_result_fn
// Provides: {"macro_8608"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `panic!` or assertions in a function whose return type is `Result`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " For some codebases, it is desirable for functions of type result to return an error instead of crashing. Hence panicking macros should be avoided."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Functions called from a function returning a `Result` may invoke a panicking macro. This is not checked."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn result_with_panic() -> Result<bool, String>"] # [doc = " {"] # [doc = "     panic!(\"error\");"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn result_without_panic() -> Result<bool, String> {"] # [doc = "     Err(String::from(\"error\"))"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.48.0"] pub PANIC_IN_RESULT_FN , restriction , "functions of type `Result<..>` that contain `panic!()` or assertion" }
};
}
