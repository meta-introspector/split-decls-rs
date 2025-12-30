// Generated macro for macro_8523 (macro)
macro_rules! Depcrate_operatorsmacro_8523 {
() => {
// Module: crate::operators
// Provides: {"macro_8523"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks any kind of arithmetic operation of any type."] # [doc = ""] # [doc = " Operators like `+`, `-`, `*` or `<<` are usually capable of overflowing according to the [Rust"] # [doc = " Reference](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow),"] # [doc = " or can panic (`/`, `%`)."] # [doc = ""] # [doc = " Known safe built-in types like `Wrapping` or `Saturating`, floats, operations in constant"] # [doc = " environments, allowed types and non-constant operations that won't overflow are ignored."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " For integers, overflow will trigger a panic in debug builds or wrap the result in"] # [doc = " release mode; division by zero will cause a panic in either mode. As a result, it is"] # [doc = " desirable to explicitly call checked, wrapping or saturating arithmetic methods."] # [doc = ""] # [doc = " #### Example"] # [doc = " ```no_run"] # [doc = " // `n` can be any number, including `i32::MAX`."] # [doc = " fn foo(n: i32) -> i32 {"] # [doc = "     n + 1"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Third-party types can also overflow or present unwanted side-effects."] # [doc = ""] # [doc = " #### Example"] # [doc = " ```ignore,rust"] # [doc = " use rust_decimal::Decimal;"] # [doc = " let _n = Decimal::MAX + Decimal::MAX;"] # [doc = " ```"] # [clippy :: version = "1.64.0"] pub ARITHMETIC_SIDE_EFFECTS , restriction , "any arithmetic expression that can cause side effects like overflows or panics" }
};
}
