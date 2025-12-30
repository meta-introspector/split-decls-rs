// Generated macro for macro_7172 (macro)
macro_rules! Depcrate_methodsmacro_7172 {
() => {
// Module: crate::methods
// Provides: {"macro_7172"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to [`splitn`]"] # [doc = " (https://doc.rust-lang.org/std/primitive.str.html#method.splitn) and"] # [doc = " related functions with either zero or one splits."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " These calls don't actually split the value and are"] # [doc = " likely to be intended as a different number."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let s = \"\";"] # [doc = " for x in s.splitn(1, \":\") {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let s = \"\";"] # [doc = " for x in s.splitn(2, \":\") {"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.54.0"] pub SUSPICIOUS_SPLITN , correctness , "checks for `.splitn(0, ..)` and `.splitn(1, ..)`" }
};
}
