// Generated macro for macro_10833 (macro)
macro_rules! Depcrate_unwrapmacro_10833 {
() => {
// Module: crate::unwrap
// Provides: {"macro_10833"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls of `unwrap[_err]()` that will always fail."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " If panicking is desired, an explicit `panic!()` should be used."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " This lint only checks `if` conditions not assignments."] # [doc = " So something like `let x: Option<()> = None; x.unwrap();` will not be recognized."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let option = Some(0);"] # [doc = " # fn do_something_with(_x: usize) {}"] # [doc = " if option.is_none() {"] # [doc = "     do_something_with(option.unwrap())"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This code will always panic. The if condition should probably be inverted."] # [clippy :: version = "pre 1.29.0"] pub PANICKING_UNWRAP , correctness , "checks for calls of `unwrap[_err]()` that will always fail" }
};
}
