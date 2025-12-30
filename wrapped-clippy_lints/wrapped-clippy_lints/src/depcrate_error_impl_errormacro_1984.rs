// Generated macro for macro_1984 (macro)
macro_rules! Depcrate_error_impl_errormacro_1984 {
() => {
// Module: crate::error_impl_error
// Provides: {"macro_1984"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for types named `Error` that implement `Error`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " It can become confusing when a codebase has 20 types all named `Error`, requiring either"] # [doc = " aliasing them in the `use` statement or qualifying them like `my_module::Error`. This"] # [doc = " hinders comprehension, as it requires you to memorize every variation of importing `Error`"] # [doc = " used across a codebase."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " #[derive(Debug)]"] # [doc = " pub enum Error { ... }"] # [doc = ""] # [doc = " impl std::fmt::Display for Error { ... }"] # [doc = ""] # [doc = " impl std::error::Error for Error { ... }"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub ERROR_IMPL_ERROR , restriction , "exported types named `Error` that implement `Error`" }
};
}
