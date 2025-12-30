// Generated macro for macro_10371 (macro)
macro_rules! Depcrate_unicodemacro_10371 {
() => {
// Module: crate::unicode
// Provides: {"macro_10371"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for non-ASCII characters in string and char literals."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Yeah, we know, the 90's called and wanted their charset"] # [doc = " back. Even so, there still are editors and other programs out there that"] # [doc = " don't work well with Unicode. So if the code is meant to be used"] # [doc = " internationally, on multiple operating systems, or has other portability"] # [doc = " requirements, activating this lint could be useful."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = String::from(\"€\");"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = String::from(\"\\u{20ac}\");"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NON_ASCII_LITERAL , restriction , "using any literal non-ASCII chars in a string literal instead of using the `\\u` escape" }
};
}
