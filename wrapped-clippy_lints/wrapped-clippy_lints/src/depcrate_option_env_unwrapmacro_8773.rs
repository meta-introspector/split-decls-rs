// Generated macro for macro_8773 (macro)
macro_rules! Depcrate_option_env_unwrapmacro_8773 {
() => {
// Module: crate::option_env_unwrap
// Provides: {"macro_8773"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `option_env!(...).unwrap()` and"] # [doc = " suggests usage of the `env!` macro."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Unwrapping the result of `option_env!` will panic"] # [doc = " at run-time if the environment variable doesn't exist, whereas `env!`"] # [doc = " catches it at compile-time."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,no_run"] # [doc = " let _ = option_env!(\"HOME\").unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " Is better expressed as:"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " let _ = env!(\"HOME\");"] # [doc = " ```"] # [clippy :: version = "1.43.0"] pub OPTION_ENV_UNWRAP , correctness , "using `option_env!(...).unwrap()` to get environment variable" }
};
}
