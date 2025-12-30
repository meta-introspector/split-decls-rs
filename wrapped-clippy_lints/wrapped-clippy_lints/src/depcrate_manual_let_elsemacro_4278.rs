// Generated macro for macro_4278 (macro)
macro_rules! Depcrate_manual_let_elsemacro_4278 {
() => {
// Module: crate::manual_let_else
// Provides: {"macro_4278"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Warn of cases where `let...else` could be used"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " `let...else` provides a standard construct for this pattern"] # [doc = " that people can easily recognize. It's also more compact."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # let w = Some(0);"] # [doc = " let v = if let Some(v) = w { v } else { return };"] # [doc = " ```"] # [doc = ""] # [doc = " Could be written:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # fn main () {"] # [doc = " # let w = Some(0);"] # [doc = " let Some(v) = w else { return };"] # [doc = " # }"] # [doc = " ```"] # [clippy :: version = "1.67.0"] pub MANUAL_LET_ELSE , pedantic , "manual implementation of a let...else statement" }
};
}
