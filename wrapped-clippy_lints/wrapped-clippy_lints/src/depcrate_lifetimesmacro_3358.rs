// Generated macro for macro_3358 (macro)
macro_rules! Depcrate_lifetimesmacro_3358 {
() => {
// Module: crate::lifetimes
// Provides: {"macro_3358"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for lifetime annotations which can be replaced with anonymous lifetimes (`'_`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The additional lifetimes can make the code look more complicated."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " This lint ignores functions with `where` clauses that reference"] # [doc = " lifetimes to prevent false positives."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::str::Chars;"] # [doc = " fn f<'a>(x: &'a str) -> Chars<'a> {"] # [doc = "     x.chars()"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::str::Chars;"] # [doc = " fn f(x: &str) -> Chars<'_> {"] # [doc = "     x.chars()"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.87.0"] pub ELIDABLE_LIFETIME_NAMES , pedantic , "lifetime name that can be replaced with the anonymous lifetime" }
};
}
