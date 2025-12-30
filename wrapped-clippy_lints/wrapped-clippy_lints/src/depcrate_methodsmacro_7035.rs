// Generated macro for macro_7035 (macro)
macro_rules! Depcrate_methodsmacro_7035 {
() => {
// Module: crate::methods
// Provides: {"macro_7035"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for calls to `iter`, `iter_mut` or `into_iter` on collections containing a single item"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " It is simpler to use the once function from the standard library:"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " let a = [123].iter();"] # [doc = " let b = Some(123).into_iter();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::iter;"] # [doc = " let a = iter::once(&123);"] # [doc = " let b = iter::once(123);"] # [doc = " ```"] # [doc = ""] # [doc = " ### Known problems"] # [doc = ""] # [doc = " The type of the resulting iterator might become incompatible with its usage"] # [clippy :: version = "1.65.0"] pub ITER_ON_SINGLE_ITEMS , nursery , "Iterator for array of length 1" }
};
}
