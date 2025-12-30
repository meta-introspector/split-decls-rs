// Generated macro for macro_8637 (macro)
macro_rules! Depcrate_panicking_overflow_checksmacro_8637 {
() => {
// Module: crate::panicking_overflow_checks
// Provides: {"macro_8637"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects C-style underflow/overflow checks."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " These checks will, by default, panic in debug builds rather than check"] # [doc = " whether the operation caused an overflow."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let a = 1i32;"] # [doc = " # let b = 2i32;"] # [doc = " if a + b < a {"] # [doc = "     // handle overflow"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let a = 1i32;"] # [doc = " # let b = 2i32;"] # [doc = " if a.checked_add(b).is_none() {"] # [doc = "     // handle overflow"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Or:"] # [doc = " ```no_run"] # [doc = " # let a = 1i32;"] # [doc = " # let b = 2i32;"] # [doc = " if a.overflowing_add(b).1 {"] # [doc = "     // handle overflow"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub PANICKING_OVERFLOW_CHECKS , correctness , "overflow checks which will panic in debug mode" }
};
}
