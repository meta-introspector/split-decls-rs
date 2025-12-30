// Generated macro for macro_5015 (macro)
macro_rules! Depcrate_mem_replacemacro_5015 {
() => {
// Module: crate::mem_replace
// Provides: {"macro_5015"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `mem::replace()` on an `Option` with"] # [doc = " `None`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `Option` already has the method `take()` for"] # [doc = " taking its current value (Some(..) or None) and replacing it with"] # [doc = " `None`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::mem;"] # [doc = ""] # [doc = " let mut an_option = Some(0);"] # [doc = " let replaced = mem::replace(&mut an_option, None);"] # [doc = " ```"] # [doc = " Is better expressed with:"] # [doc = " ```no_run"] # [doc = " let mut an_option = Some(0);"] # [doc = " let taken = an_option.take();"] # [doc = " ```"] # [clippy :: version = "1.31.0"] pub MEM_REPLACE_OPTION_WITH_NONE , style , "replacing an `Option` with `None` instead of `take()`" }
};
}
