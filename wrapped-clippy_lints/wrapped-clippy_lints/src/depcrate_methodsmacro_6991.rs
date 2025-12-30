// Generated macro for macro_6991 (macro)
macro_rules! Depcrate_methodsmacro_6991 {
() => {
// Module: crate::methods
// Provides: {"macro_6991"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the use of `iter.nth(0)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `iter.next()` is equivalent to"] # [doc = " `iter.nth(0)`, as they both consume the next element,"] # [doc = "  but is more readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::collections::HashSet;"] # [doc = " # let mut s = HashSet::new();"] # [doc = " # s.insert(1);"] # [doc = " let x = s.iter().nth(0);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::collections::HashSet;"] # [doc = " # let mut s = HashSet::new();"] # [doc = " # s.insert(1);"] # [doc = " let x = s.iter().next();"] # [doc = " ```"] # [clippy :: version = "1.42.0"] pub ITER_NTH_ZERO , style , "replace `iter.nth(0)` with `iter.next()`" }
};
}
