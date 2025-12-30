// Generated macro for macro_1354 (macro)
macro_rules! Depcrate_defaultmacro_1354 {
() => {
// Module: crate::default
// Provides: {"macro_1354"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for immediate reassignment of fields initialized"] # [doc = " with Default::default()."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = "It's more idiomatic to use the [functional update syntax](https://doc.rust-lang.org/reference/expressions/struct-expr.html#functional-update-syntax)."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Assignments to patterns that are of tuple type are not linted."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # #[derive(Default)]"] # [doc = " # struct A { i: i32 }"] # [doc = " let mut a: A = Default::default();"] # [doc = " a.i = 42;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # #[derive(Default)]"] # [doc = " # struct A { i: i32 }"] # [doc = " let a = A {"] # [doc = "     i: 42,"] # [doc = "     .. Default::default()"] # [doc = " };"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub FIELD_REASSIGN_WITH_DEFAULT , style , "binding initialized with Default should have its fields set in the initializer" }
};
}
