// Generated macro for macro_3159 (macro)
macro_rules! Depcrate_iter_not_returning_iteratormacro_3159 {
() => {
// Module: crate::iter_not_returning_iterator
// Provides: {"macro_3159"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects methods named `iter` or `iter_mut` that do not have a return type that implements `Iterator`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Methods named `iter` or `iter_mut` conventionally return an `Iterator`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " // `String` does not implement `Iterator`"] # [doc = " struct Data {}"] # [doc = " impl Data {"] # [doc = "     fn iter(&self) -> String {"] # [doc = "         todo!()"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::str::Chars;"] # [doc = " struct Data {}"] # [doc = " impl Data {"] # [doc = "     fn iter(&self) -> Chars<'static> {"] # [doc = "         todo!()"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.57.0"] pub ITER_NOT_RETURNING_ITERATOR , pedantic , "methods named `iter` or `iter_mut` that do not return an `Iterator`" }
};
}
