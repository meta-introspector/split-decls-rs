// Generated macro for macro_3340 (macro)
macro_rules! Depcrate_len_zeromacro_3340 {
() => {
// Module: crate::len_zero
// Provides: {"macro_3340"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for items that implement `.len()` but not"] # [doc = " `.is_empty()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is good custom to have both methods, because for"] # [doc = " some data structures, asking about the length will be a costly operation,"] # [doc = " whereas `.is_empty()` can usually answer in constant time. Also it used to"] # [doc = " lead to false positives on the [`len_zero`](#len_zero) lint – currently that"] # [doc = " lint will ignore such entities."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " impl X {"] # [doc = "     pub fn len(&self) -> usize {"] # [doc = "         .."] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub LEN_WITHOUT_IS_EMPTY , style , "traits or impls with a public `len` method but no corresponding `is_empty` method" }
};
}
