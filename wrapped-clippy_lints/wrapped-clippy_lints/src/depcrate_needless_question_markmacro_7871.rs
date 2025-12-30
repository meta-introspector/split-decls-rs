// Generated macro for macro_7871 (macro)
macro_rules! Depcrate_needless_question_markmacro_7871 {
() => {
// Module: crate::needless_question_mark
// Provides: {"macro_7871"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Suggests replacing `Ok(x?)` or `Some(x?)` with `x` in return positions where the `?` operator"] # [doc = " is not needed to convert the type of `x`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " There's no reason to use `?` to short-circuit when execution of the body will end there anyway."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::num::ParseIntError;"] # [doc = " fn f(s: &str) -> Option<usize> {"] # [doc = "     Some(s.find('x')?)"] # [doc = " }"] # [doc = ""] # [doc = " fn g(s: &str) -> Result<usize, ParseIntError> {"] # [doc = "     Ok(s.parse()?)"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::num::ParseIntError;"] # [doc = " fn f(s: &str) -> Option<usize> {"] # [doc = "     s.find('x')"] # [doc = " }"] # [doc = ""] # [doc = " fn g(s: &str) -> Result<usize, ParseIntError> {"] # [doc = "     s.parse()"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub NEEDLESS_QUESTION_MARK , complexity , "using `Ok(x?)` or `Some(x?)` where `x` would be equivalent" }
};
}
