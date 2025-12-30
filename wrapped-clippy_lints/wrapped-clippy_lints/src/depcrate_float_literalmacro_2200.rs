// Generated macro for macro_2200 (macro)
macro_rules! Depcrate_float_literalmacro_2200 {
() => {
// Module: crate::float_literal
// Provides: {"macro_2200"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for float literals with a precision greater"] # [doc = " than that supported by the underlying type."] # [doc = ""] # [doc = " The lint is suppressed for literals with over `const_literal_digits_threshold` digits."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Rust will truncate the literal silently."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let v: f32 = 0.123_456_789_9;"] # [doc = " println!(\"{}\", v); //  0.123_456_789"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let v: f64 = 0.123_456_789_9;"] # [doc = " println!(\"{}\", v); //  0.123_456_789_9"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub EXCESSIVE_PRECISION , style , "excessive precision for float literal" }
};
}
