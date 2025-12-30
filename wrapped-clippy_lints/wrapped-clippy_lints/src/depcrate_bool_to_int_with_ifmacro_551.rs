// Generated macro for macro_551 (macro)
macro_rules! Depcrate_bool_to_int_with_ifmacro_551 {
() => {
// Module: crate::bool_to_int_with_if
// Provides: {"macro_551"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Instead of using an if statement to convert a bool to an int,"] # [doc = " this lint suggests using a `from()` function or an `as` coercion."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Coercion or `from()` is another way to convert bool to a number."] # [doc = " Both methods are guaranteed to return 1 for true, and 0 for false."] # [doc = ""] # [doc = " See https://doc.rust-lang.org/std/primitive.bool.html#impl-From%3Cbool%3E"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let condition = false;"] # [doc = " if condition {"] # [doc = "     1_i64"] # [doc = " } else {"] # [doc = "     0"] # [doc = " };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let condition = false;"] # [doc = " i64::from(condition);"] # [doc = " ```"] # [doc = " or"] # [doc = " ```no_run"] # [doc = " # let condition = false;"] # [doc = " condition as i64;"] # [doc = " ```"] # [clippy :: version = "1.65.0"] pub BOOL_TO_INT_WITH_IF , pedantic , "using if to convert bool to int" }
};
}
