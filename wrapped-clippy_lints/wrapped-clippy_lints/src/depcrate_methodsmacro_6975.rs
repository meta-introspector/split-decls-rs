// Generated macro for macro_6975 (macro)
macro_rules! Depcrate_methodsmacro_6975 {
() => {
// Module: crate::methods
// Provides: {"macro_6975"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_.map(_).flatten(_)` on `Iterator` and `Option`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more concisely as"] # [doc = " `_.flat_map(_)` for `Iterator` or `_.and_then(_)` for `Option`"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let vec = vec![vec![1]];"] # [doc = " let opt = Some(5);"] # [doc = ""] # [doc = " vec.iter().map(|x| x.iter()).flatten();"] # [doc = " opt.map(|x| Some(x * 2)).flatten();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let vec = vec![vec![1]];"] # [doc = " # let opt = Some(5);"] # [doc = " vec.iter().flat_map(|x| x.iter());"] # [doc = " opt.and_then(|x| Some(x * 2));"] # [doc = " ```"] # [clippy :: version = "1.31.0"] pub MAP_FLATTEN , complexity , "using combinations of `flatten` and `map` which can usually be written as a single method call" }
};
}
