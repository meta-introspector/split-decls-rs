// Generated macro for macro_10167 (macro)
macro_rules! Depcrate_time_subtractionmacro_10167 {
() => {
// Module: crate::time_subtraction
// Provides: {"macro_10167"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Lints subtraction between `Instant::now()` and another `Instant`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is easy to accidentally write `prev_instant - Instant::now()`, which will always be 0ns"] # [doc = " as `Instant` subtraction saturates."] # [doc = ""] # [doc = " `prev_instant.elapsed()` also more clearly signals intention."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::time::Instant;"] # [doc = " let prev_instant = Instant::now();"] # [doc = " let duration = Instant::now() - prev_instant;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::time::Instant;"] # [doc = " let prev_instant = Instant::now();"] # [doc = " let duration = prev_instant.elapsed();"] # [doc = " ```"] # [clippy :: version = "1.65.0"] pub MANUAL_INSTANT_ELAPSED , pedantic , "subtraction between `Instant::now()` and previous `Instant`" }
};
}
