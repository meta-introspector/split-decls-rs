// Generated macro for macro_9868 (macro)
macro_rules! Depcrate_to_digit_is_somemacro_9868 {
() => {
// Module: crate::to_digit_is_some
// Provides: {"macro_9868"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `.to_digit(..).is_some()` on `char`s."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is a convoluted way of checking if a `char` is a digit. It's"] # [doc = " more straight forward to use the dedicated `is_digit` method."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let c = 'c';"] # [doc = " # let radix = 10;"] # [doc = " let is_digit = c.to_digit(radix).is_some();"] # [doc = " ```"] # [doc = " can be written as:"] # [doc = " ```no_run"] # [doc = " # let c = 'c';"] # [doc = " # let radix = 10;"] # [doc = " let is_digit = c.is_digit(radix);"] # [doc = " ```"] # [clippy :: version = "1.41.0"] pub TO_DIGIT_IS_SOME , style , "`char.is_digit()` is clearer" }
};
}
