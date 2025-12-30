// Generated macro for macro_4379 (macro)
macro_rules! Depcrate_manual_rem_euclidmacro_4379 {
() => {
// Module: crate::manual_rem_euclid
// Provides: {"macro_4379"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for an expression like `((x % 4) + 4) % 4` which is a common manual reimplementation"] # [doc = " of `x.rem_euclid(4)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's simpler and more readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x: i32 = 24;"] # [doc = " let rem = ((x % 4) + 4) % 4;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x: i32 = 24;"] # [doc = " let rem = x.rem_euclid(4);"] # [doc = " ```"] # [clippy :: version = "1.64.0"] pub MANUAL_REM_EUCLID , complexity , "manually reimplementing `rem_euclid`" }
};
}
