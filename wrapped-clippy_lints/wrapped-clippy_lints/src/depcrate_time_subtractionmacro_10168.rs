// Generated macro for macro_10168 (macro)
macro_rules! Depcrate_time_subtractionmacro_10168 {
() => {
// Module: crate::time_subtraction
// Provides: {"macro_10168"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Lints subtraction between an `Instant` and a `Duration`, or between two `Duration` values."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Unchecked subtraction could cause underflow on certain platforms, leading to"] # [doc = " unintentional panics."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::time::{Instant, Duration};"] # [doc = " let time_passed = Instant::now() - Duration::from_secs(5);"] # [doc = " let dur1 = Duration::from_secs(3);"] # [doc = " let dur2 = Duration::from_secs(5);"] # [doc = " let diff = dur1 - dur2;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::time::{Instant, Duration};"] # [doc = " let time_passed = Instant::now().checked_sub(Duration::from_secs(5));"] # [doc = " let dur1 = Duration::from_secs(3);"] # [doc = " let dur2 = Duration::from_secs(5);"] # [doc = " let diff = dur1.checked_sub(dur2);"] # [doc = " ```"] # [clippy :: version = "1.67.0"] pub UNCHECKED_TIME_SUBTRACTION , pedantic , "finds unchecked subtraction involving 'Duration' or 'Instant'" }
};
}
