// Generated macro for macro_3902 (macro)
macro_rules! Depcrate_loopsmacro_3902 {
() => {
// Module: crate::loops
// Provides: {"macro_3902"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for unnecessary `if let` usage in a for loop"] # [doc = " where only the `Some` or `Ok` variant of the iterator element is used."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is verbose and can be simplified"] # [doc = " by first calling the `flatten` method on the `Iterator`."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " let x = vec![Some(1), Some(2), Some(3)];"] # [doc = " for n in x {"] # [doc = "     if let Some(n) = n {"] # [doc = "         println!(\"{}\", n);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = vec![Some(1), Some(2), Some(3)];"] # [doc = " for n in x.into_iter().flatten() {"] # [doc = "     println!(\"{}\", n);"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.52.0"] pub MANUAL_FLATTEN , complexity , "for loops over `Option`s or `Result`s with a single expression can be simplified" }
};
}
