// Generated macro for macro_7006 (macro)
macro_rules! Depcrate_methodsmacro_7006 {
() => {
// Module: crate::methods
// Provides: {"macro_7006"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `map` followed by a `count`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It looks suspicious. Maybe `map` was confused with `filter`."] # [doc = " If the `map` call is intentional, this should be rewritten"] # [doc = " using `inspect`. Or, if you intend to drive the iterator to"] # [doc = " completion, you can just use `for_each` instead."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = (0..3).map(|x| x + 2).count();"] # [doc = " ```"] # [clippy :: version = "1.39.0"] pub SUSPICIOUS_MAP , suspicious , "suspicious usage of map" }
};
}
