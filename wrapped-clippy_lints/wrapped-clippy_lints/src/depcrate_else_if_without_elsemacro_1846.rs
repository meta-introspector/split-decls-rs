// Generated macro for macro_1846 (macro)
macro_rules! Depcrate_else_if_without_elsemacro_1846 {
() => {
// Module: crate::else_if_without_else
// Provides: {"macro_1846"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of if expressions with an `else if` branch,"] # [doc = " but without a final `else` branch."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Some coding guidelines require this (e.g., MISRA-C:2004 Rule 14.10)."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # fn a() {}"] # [doc = " # fn b() {}"] # [doc = " # let x: i32 = 1;"] # [doc = " if x.is_positive() {"] # [doc = "     a();"] # [doc = " } else if x.is_negative() {"] # [doc = "     b();"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # fn a() {}"] # [doc = " # fn b() {}"] # [doc = " # let x: i32 = 1;"] # [doc = " if x.is_positive() {"] # [doc = "     a();"] # [doc = " } else if x.is_negative() {"] # [doc = "     b();"] # [doc = " } else {"] # [doc = "     // We don't care about zero."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ELSE_IF_WITHOUT_ELSE , restriction , "`if` expression with an `else if`, but without a final `else` branch" }
};
}
