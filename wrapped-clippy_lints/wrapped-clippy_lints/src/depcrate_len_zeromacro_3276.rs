// Generated macro for macro_3276 (macro)
macro_rules! Depcrate_len_zeromacro_3276 {
() => {
// Module: crate::len_zero
// Provides: {"macro_3276"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for getting the length of something via `.len()`"] # [doc = " just to compare to zero, and suggests using `.is_empty()` where applicable."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Some structures can answer `.is_empty()` much faster"] # [doc = " than calculating their length. So it is good to get into the habit of using"] # [doc = " `.is_empty()`, and having it is cheap."] # [doc = " Besides, it makes the intent clearer than a manual comparison in some contexts."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " if x.len() == 0 {"] # [doc = "     .."] # [doc = " }"] # [doc = " if y.len() != 0 {"] # [doc = "     .."] # [doc = " }"] # [doc = " ```"] # [doc = " instead use"] # [doc = " ```ignore"] # [doc = " if x.is_empty() {"] # [doc = "     .."] # [doc = " }"] # [doc = " if !y.is_empty() {"] # [doc = "     .."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub LEN_ZERO , style , "checking `.len() == 0` or `.len() > 0` (or similar) when `.is_empty()` could be used instead" }
};
}
