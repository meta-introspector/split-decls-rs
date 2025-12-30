// Generated macro for macro_8533 (macro)
macro_rules! Depcrate_operatorsmacro_8533 {
() => {
// Module: crate::operators
// Provides: {"macro_8533"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calculation of subsecond microseconds or milliseconds"] # [doc = " from other `Duration` methods."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's more concise to call `Duration::subsec_micros()` or"] # [doc = " `Duration::subsec_millis()` than to calculate them."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::time::Duration;"] # [doc = " # let duration = Duration::new(5, 0);"] # [doc = " let micros = duration.subsec_nanos() / 1_000;"] # [doc = " let millis = duration.subsec_nanos() / 1_000_000;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::time::Duration;"] # [doc = " # let duration = Duration::new(5, 0);"] # [doc = " let micros = duration.subsec_micros();"] # [doc = " let millis = duration.subsec_millis();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub DURATION_SUBSEC , complexity , "checks for calculation of subsecond microseconds or milliseconds" }
};
}
