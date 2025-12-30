// Generated macro for macro_7155 (macro)
macro_rules! Depcrate_minmaxmacro_7155 {
() => {
// Module: crate::minmax
// Provides: {"macro_7155"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for expressions where `std::cmp::min` and `max` are"] # [doc = " used to clamp values, but switched so that the result is constant."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is in all probability not the intended outcome. At"] # [doc = " the least it hurts readability of the code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " min(0, max(100, x))"] # [doc = ""] # [doc = " // or"] # [doc = ""] # [doc = " x.max(100).min(0)"] # [doc = " ```"] # [doc = " It will always be equal to `0`. Probably the author meant to clamp the value"] # [doc = " between 0 and 100, but has erroneously swapped `min` and `max`."] # [clippy :: version = "pre 1.29.0"] pub MIN_MAX , correctness , "`min(_, max(_, _))` (or vice versa) with bounds clamping the result to a constant" }
};
}
