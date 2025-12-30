// Generated macro for macro_7031 (macro)
macro_rules! Depcrate_methodsmacro_7031 {
() => {
// Module: crate::methods
// Provides: {"macro_7031"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Finds usages of [`char::is_digit`](https://doc.rust-lang.org/stable/std/primitive.char.html#method.is_digit) that"] # [doc = " can be replaced with [`is_ascii_digit`](https://doc.rust-lang.org/stable/std/primitive.char.html#method.is_ascii_digit) or"] # [doc = " [`is_ascii_hexdigit`](https://doc.rust-lang.org/stable/std/primitive.char.html#method.is_ascii_hexdigit)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `is_digit(..)` is slower and requires specifying the radix."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let c: char = '6';"] # [doc = " c.is_digit(10);"] # [doc = " c.is_digit(16);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let c: char = '6';"] # [doc = " c.is_ascii_digit();"] # [doc = " c.is_ascii_hexdigit();"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub IS_DIGIT_ASCII_RADIX , style , "use of `char::is_digit(..)` with literal radix of 10 or 16" }
};
}
