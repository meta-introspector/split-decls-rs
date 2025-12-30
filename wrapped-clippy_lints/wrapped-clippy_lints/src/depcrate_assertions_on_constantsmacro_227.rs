// Generated macro for macro_227 (macro)
macro_rules! Depcrate_assertions_on_constantsmacro_227 {
() => {
// Module: crate::assertions_on_constants
// Provides: {"macro_227"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `assert!(true)` and `assert!(false)` calls."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Will be optimized out by the compiler or should probably be replaced by a"] # [doc = " `panic!()` or `unreachable!()`"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " assert!(false)"] # [doc = " assert!(true)"] # [doc = " const B: bool = false;"] # [doc = " assert!(B)"] # [doc = " ```"] # [clippy :: version = "1.34.0"] pub ASSERTIONS_ON_CONSTANTS , style , "`assert!(true)` / `assert!(false)` will be optimized out by the compiler, and should probably be replaced by a `panic!()` or `unreachable!()`" }
};
}
