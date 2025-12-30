// Generated macro for macro_7036 (macro)
macro_rules! Depcrate_methodsmacro_7036 {
() => {
// Module: crate::methods
// Provides: {"macro_7036"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for calls to `iter`, `iter_mut` or `into_iter` on empty collections"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " It is simpler to use the empty function from the standard library:"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::{slice, option};"] # [doc = " let a: slice::Iter<i32> = [].iter();"] # [doc = " let f: option::IntoIter<i32> = None.into_iter();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::iter;"] # [doc = " let a: iter::Empty<i32> = iter::empty();"] # [doc = " let b: iter::Empty<i32> = iter::empty();"] # [doc = " ```"] # [doc = ""] # [doc = " ### Known problems"] # [doc = ""] # [doc = " The type of the resulting iterator might become incompatible with its usage"] # [clippy :: version = "1.65.0"] pub ITER_ON_EMPTY_COLLECTIONS , nursery , "Iterator for empty array" }
};
}
