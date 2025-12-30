// Generated macro for macro_8529 (macro)
macro_rules! Depcrate_operatorsmacro_8529 {
() => {
// Module: crate::operators
// Provides: {"macro_8529"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for bit masks that can be replaced by a call"] # [doc = " to `trailing_zeros`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `x.trailing_zeros() >= 4` is much clearer than `x & 15"] # [doc = " == 0`"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let x = 1;"] # [doc = " if x & 0b1111 == 0 { }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let x: i32 = 1;"] # [doc = " if x.trailing_zeros() >= 4 { }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub VERBOSE_BIT_MASK , pedantic , "expressions where a bit mask is less readable than the corresponding method call" }
};
}
