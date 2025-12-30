// Generated macro for macro_4533 (macro)
macro_rules! Depcrate_match_result_okmacro_4533 {
() => {
// Module: crate::match_result_ok
// Provides: {"macro_4533"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for unnecessary `ok()` in `while let`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Calling `ok()` in `while let` is unnecessary, instead match"] # [doc = " on `Ok(pat)`"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " while let Some(value) = iter.next().ok() {"] # [doc = "     vec.push(value)"] # [doc = " }"] # [doc = ""] # [doc = " if let Some(value) = iter.next().ok() {"] # [doc = "     vec.push(value)"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```ignore"] # [doc = " while let Ok(value) = iter.next() {"] # [doc = "     vec.push(value)"] # [doc = " }"] # [doc = ""] # [doc = " if let Ok(value) = iter.next() {"] # [doc = "        vec.push(value)"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.57.0"] pub MATCH_RESULT_OK , style , "usage of `ok()` in `let Some(pat)` statements is unnecessary, match on `Ok(pat)` instead" }
};
}
