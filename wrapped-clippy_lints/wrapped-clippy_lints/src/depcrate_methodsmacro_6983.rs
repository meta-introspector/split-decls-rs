// Generated macro for macro_6983 (macro)
macro_rules! Depcrate_methodsmacro_6983 {
() => {
// Module: crate::methods
// Provides: {"macro_6983"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `.or(…).unwrap()` calls to Options and Results."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " You should use `.unwrap_or(…)` instead for clarity."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let fallback = \"fallback\";"] # [doc = " // Result"] # [doc = " # type Error = &'static str;"] # [doc = " # let result: Result<&str, Error> = Err(\"error\");"] # [doc = " let value = result.or::<Error>(Ok(fallback)).unwrap();"] # [doc = ""] # [doc = " // Option"] # [doc = " # let option: Option<&str> = None;"] # [doc = " let value = option.or(Some(fallback)).unwrap();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let fallback = \"fallback\";"] # [doc = " // Result"] # [doc = " # let result: Result<&str, &str> = Err(\"error\");"] # [doc = " let value = result.unwrap_or(fallback);"] # [doc = ""] # [doc = " // Option"] # [doc = " # let option: Option<&str> = None;"] # [doc = " let value = option.unwrap_or(fallback);"] # [doc = " ```"] # [clippy :: version = "1.61.0"] pub OR_THEN_UNWRAP , complexity , "checks for `.or(…).unwrap()` calls to Options and Results." }
};
}
