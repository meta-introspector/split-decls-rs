// Generated macro for macro_128 (macro)
macro_rules! Depcrate_almost_complete_rangemacro_128 {
() => {
// Module: crate::almost_complete_range
// Provides: {"macro_128"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for ranges which almost include the entire range of letters from 'a' to 'z'"] # [doc = " or digits from '0' to '9', but don't because they're a half open range."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This (`'a'..'z'`) is almost certainly a typo meant to include all letters."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = 'a'..'z';"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let _ = 'a'..='z';"] # [doc = " ```"] # [clippy :: version = "1.68.0"] pub ALMOST_COMPLETE_RANGE , suspicious , "almost complete range" }
};
}
