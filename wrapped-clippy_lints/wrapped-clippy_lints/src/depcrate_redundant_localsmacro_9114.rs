// Generated macro for macro_9114 (macro)
macro_rules! Depcrate_redundant_localsmacro_9114 {
() => {
// Module: crate::redundant_locals
// Provides: {"macro_9114"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for redundant redefinitions of local bindings."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Redundant redefinitions of local bindings do not change behavior other than variable's lifetimes and are likely to be unintended."] # [doc = ""] # [doc = " These rebindings can be intentional to shorten the lifetimes of variables because they affect when the `Drop` implementation is called. Other than that, they do not affect your code's meaning but they _may_ affect `rustc`'s stack allocation."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let a = 0;"] # [doc = " let a = a;"] # [doc = ""] # [doc = " fn foo(b: i32) {"] # [doc = "     let b = b;"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let a = 0;"] # [doc = " // no redefinition with the same name"] # [doc = ""] # [doc = " fn foo(b: i32) {"] # [doc = "   // no redefinition with the same name"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub REDUNDANT_LOCALS , suspicious , "redundant redefinition of a local binding" }
};
}
