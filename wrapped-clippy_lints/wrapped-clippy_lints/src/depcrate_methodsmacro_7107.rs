// Generated macro for macro_7107 (macro)
macro_rules! Depcrate_methodsmacro_7107 {
() => {
// Module: crate::methods
// Provides: {"macro_7107"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `Iterator::flat_map()` where `filter_map()` could be"] # [doc = " used instead."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `filter_map()` is known to always produce 0 or 1 output items per input item,"] # [doc = " rather than however many the inner iterator type produces."] # [doc = " Therefore, it maintains the upper bound in `Iterator::size_hint()`,"] # [doc = " and communicates to the reader that the input items are not being expanded into"] # [doc = " multiple output items without their having to notice that the mapping function"] # [doc = " returns an `Option`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let nums: Vec<i32> = [\"1\", \"2\", \"whee!\"].iter().flat_map(|x| x.parse().ok()).collect();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let nums: Vec<i32> = [\"1\", \"2\", \"whee!\"].iter().filter_map(|x| x.parse().ok()).collect();"] # [doc = " ```"] # [clippy :: version = "1.53.0"] pub FLAT_MAP_OPTION , pedantic , "used `flat_map` where `filter_map` could be used instead" }
};
}
