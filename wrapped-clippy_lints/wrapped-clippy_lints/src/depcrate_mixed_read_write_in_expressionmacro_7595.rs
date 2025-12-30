// Generated macro for macro_7595 (macro)
macro_rules! Depcrate_mixed_read_write_in_expressionmacro_7595 {
() => {
// Module: crate::mixed_read_write_in_expression
// Provides: {"macro_7595"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for diverging calls that are not match arms or"] # [doc = " statements."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is often confusing to read. In addition, the"] # [doc = " sub-expression evaluation order for Rust is not well documented."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Someone might want to use `some_bool || panic!()` as a"] # [doc = " shorthand."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,no_run"] # [doc = " # fn b() -> bool { true }"] # [doc = " # fn c() -> bool { true }"] # [doc = " let a = b() || panic!() || c();"] # [doc = " // `c()` is dead, `panic!()` is only called if `b()` returns `false`"] # [doc = " let x = (a, b, c, panic!());"] # [doc = " // can simply be replaced by `panic!()`"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub DIVERGING_SUB_EXPRESSION , complexity , "whether an expression contains a diverging sub expression" }
};
}
