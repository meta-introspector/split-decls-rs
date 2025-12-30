// Generated macro for macro_2356 (macro)
macro_rules! Depcrate_formattingmacro_2356 {
() => {
// Module: crate::formatting
// Provides: {"macro_2356"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for formatting of `else`. It lints if the `else`"] # [doc = " is followed immediately by a newline or the `else` seems to be missing."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is probably some refactoring remnant, even if the"] # [doc = " code is correct, it might look confusing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " if foo {"] # [doc = " } { // looks like an `else` is missing here"] # [doc = " }"] # [doc = ""] # [doc = " if foo {"] # [doc = " } if bar { // looks like an `else` is missing here"] # [doc = " }"] # [doc = ""] # [doc = " if foo {"] # [doc = " } else"] # [doc = ""] # [doc = " { // this is the `else` block of the previous `if`, but should it be?"] # [doc = " }"] # [doc = ""] # [doc = " if foo {"] # [doc = " } else"] # [doc = ""] # [doc = " if bar { // this is the `else` block of the previous `if`, but should it be?"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub SUSPICIOUS_ELSE_FORMATTING , suspicious , "suspicious formatting of `else`" }
};
}
