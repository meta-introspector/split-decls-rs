// Generated macro for macro_1128 (macro)
macro_rules! Depcrate_checked_conversionsmacro_1128 {
() => {
// Module: crate::checked_conversions
// Provides: {"macro_1128"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for explicit bounds checking when casting."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Reduces the readability of statements & is error prone."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let foo: u32 = 5;"] # [doc = " foo <= i32::MAX as u32;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let foo = 1;"] # [doc = " # #[allow(unused)]"] # [doc = " i32::try_from(foo).is_ok();"] # [doc = " ```"] # [clippy :: version = "1.37.0"] pub CHECKED_CONVERSIONS , pedantic , "`try_from` could replace manual bounds checking when casting" }
};
}
