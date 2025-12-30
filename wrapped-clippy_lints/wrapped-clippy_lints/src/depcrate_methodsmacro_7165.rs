// Generated macro for macro_7165 (macro)
macro_rules! Depcrate_methodsmacro_7165 {
() => {
// Module: crate::methods
// Provides: {"macro_7165"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `inspect().for_each()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is the same as performing the computation"] # [doc = " inside `inspect` at the beginning of the closure in `for_each`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " [1,2,3,4,5].iter()"] # [doc = " .inspect(|&x| println!(\"inspect the number: {}\", x))"] # [doc = " .for_each(|&x| {"] # [doc = "     assert!(x >= 0);"] # [doc = " });"] # [doc = " ```"] # [doc = " Can be written as"] # [doc = " ```no_run"] # [doc = " [1,2,3,4,5].iter()"] # [doc = " .for_each(|&x| {"] # [doc = "     println!(\"inspect the number: {}\", x);"] # [doc = "     assert!(x >= 0);"] # [doc = " });"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub INSPECT_FOR_EACH , complexity , "using `.inspect().for_each()`, which can be replaced with `.for_each()`" }
};
}
