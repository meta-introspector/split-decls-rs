// Generated macro for macro_6978 (macro)
macro_rules! Depcrate_methodsmacro_6978 {
() => {
// Module: crate::methods
// Provides: {"macro_6978"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_.filter_map(_).next()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability, this can be written more concisely as"] # [doc = " `_.find_map(_)`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = "  (0..3).filter_map(|x| if x == 2 { Some(x) } else { None }).next();"] # [doc = " ```"] # [doc = " Can be written as"] # [doc = ""] # [doc = " ```no_run"] # [doc = "  (0..3).find_map(|x| if x == 2 { Some(x) } else { None });"] # [doc = " ```"] # [clippy :: version = "1.36.0"] pub FILTER_MAP_NEXT , pedantic , "using combination of `filter_map` and `next` which can usually be written as a single method call" }
};
}
