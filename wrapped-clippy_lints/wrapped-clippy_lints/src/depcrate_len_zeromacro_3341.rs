// Generated macro for macro_3341 (macro)
macro_rules! Depcrate_len_zeromacro_3341 {
() => {
// Module: crate::len_zero
// Provides: {"macro_3341"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for comparing to an empty slice such as `\"\"` or `[]`,"] # [doc = " and suggests using `.is_empty()` where applicable."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Some structures can answer `.is_empty()` much faster"] # [doc = " than checking for equality. So it is good to get into the habit of using"] # [doc = " `.is_empty()`, and having it is cheap."] # [doc = " Besides, it makes the intent clearer than a manual comparison in some contexts."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```ignore"] # [doc = " if s == \"\" {"] # [doc = "     .."] # [doc = " }"] # [doc = ""] # [doc = " if arr == [] {"] # [doc = "     .."] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```ignore"] # [doc = " if s.is_empty() {"] # [doc = "     .."] # [doc = " }"] # [doc = ""] # [doc = " if arr.is_empty() {"] # [doc = "     .."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub COMPARISON_TO_EMPTY , style , "checking `x == \"\"` or `x == []` (or similar) when `.is_empty()` could be used instead" }
};
}
