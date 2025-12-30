// Generated macro for macro_3949 (macro)
macro_rules! Depcrate_loopsmacro_3949 {
() => {
// Module: crate::loops
// Provides: {"macro_3949"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `while let` expressions on iterators."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability. A simple `for` loop is shorter and conveys"] # [doc = " the intent better."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " while let Some(val) = iter.next() {"] # [doc = "     .."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```ignore"] # [doc = " for val in &mut iter {"] # [doc = "     .."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub WHILE_LET_ON_ITERATOR , style , "using a `while let` loop instead of a for loop on an iterator" }
};
}
