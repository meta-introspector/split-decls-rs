// Generated macro for macro_5016 (macro)
macro_rules! Depcrate_mem_replacemacro_5016 {
() => {
// Module: crate::mem_replace
// Provides: {"macro_5016"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `mem::replace()` on an `Option` with `Some(…)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `Option` already has the method `replace()` for"] # [doc = " taking its current value (Some(…) or None) and replacing it with"] # [doc = " `Some(…)`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut an_option = Some(0);"] # [doc = " let replaced = std::mem::replace(&mut an_option, Some(1));"] # [doc = " ```"] # [doc = " Is better expressed with:"] # [doc = " ```no_run"] # [doc = " let mut an_option = Some(0);"] # [doc = " let taken = an_option.replace(1);"] # [doc = " ```"] # [clippy :: version = "1.87.0"] pub MEM_REPLACE_OPTION_WITH_SOME , style , "replacing an `Option` with `Some` instead of `replace()`" }
};
}
