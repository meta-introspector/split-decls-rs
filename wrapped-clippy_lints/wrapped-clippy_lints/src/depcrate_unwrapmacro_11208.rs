// Generated macro for macro_11208 (macro)
macro_rules! Depcrate_unwrapmacro_11208 {
() => {
// Module: crate::unwrap
// Provides: {"macro_11208"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls of `unwrap[_err]()` that cannot fail."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `if let` or `match` is more idiomatic."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let option = Some(0);"] # [doc = " # fn do_something_with(_x: usize) {}"] # [doc = " if option.is_some() {"] # [doc = "     do_something_with(option.unwrap())"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Could be written:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let option = Some(0);"] # [doc = " # fn do_something_with(_x: usize) {}"] # [doc = " if let Some(value) = option {"] # [doc = "     do_something_with(value)"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub UNNECESSARY_UNWRAP , complexity , "checks for calls of `unwrap[_err]()` that cannot fail" }
};
}
