// Generated macro for macro_9235 (macro)
macro_rules! Depcrate_referencemacro_9235 {
() => {
// Module: crate::reference
// Provides: {"macro_9235"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `*&` and `*&mut` in expressions."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Immediately dereferencing a reference is no-op and"] # [doc = " makes the code less clear."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Multiple dereference/addrof pairs are not handled so"] # [doc = " the suggested fix for `x = **&&y` is `x = *&y`, which is still incorrect."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let a = f(*&mut b);"] # [doc = " let c = *&d;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " let a = f(b);"] # [doc = " let c = d;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub DEREF_ADDROF , complexity , "use of `*&` or `*&mut` in an expression" }
};
}
