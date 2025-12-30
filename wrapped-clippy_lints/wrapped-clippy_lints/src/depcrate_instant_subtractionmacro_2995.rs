// Generated macro for macro_2995 (macro)
macro_rules! Depcrate_instant_subtractionmacro_2995 {
() => {
// Module: crate::instant_subtraction
// Provides: {"macro_2995"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Lints subtraction between an `Instant` and a `Duration`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Unchecked subtraction could cause underflow on certain platforms, leading to"] # [doc = " unintentional panics."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::time::{Instant, Duration};"] # [doc = " let time_passed = Instant::now() - Duration::from_secs(5);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::time::{Instant, Duration};"] # [doc = " let time_passed = Instant::now().checked_sub(Duration::from_secs(5));"] # [doc = " ```"] # [clippy :: version = "1.67.0"] pub UNCHECKED_DURATION_SUBTRACTION , pedantic , "finds unchecked subtraction of a 'Duration' from an 'Instant'" }
};
}
