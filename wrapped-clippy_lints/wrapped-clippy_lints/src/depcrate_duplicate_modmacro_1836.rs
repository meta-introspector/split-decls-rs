// Generated macro for macro_1836 (macro)
macro_rules! Depcrate_duplicate_modmacro_1836 {
() => {
// Module: crate::duplicate_mod
// Provides: {"macro_1836"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for files that are included as modules multiple times."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Loading a file as a module more than once causes it to be compiled"] # [doc = " multiple times, taking longer and putting duplicate content into the"] # [doc = " module tree."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " // lib.rs"] # [doc = " mod a;"] # [doc = " mod b;"] # [doc = " ```"] # [doc = " ```rust,ignore"] # [doc = " // a.rs"] # [doc = " #[path = \"./b.rs\"]"] # [doc = " mod b;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " // lib.rs"] # [doc = " mod a;"] # [doc = " mod b;"] # [doc = " ```"] # [doc = " ```rust,ignore"] # [doc = " // a.rs"] # [doc = " use crate::b;"] # [doc = " ```"] # [clippy :: version = "1.63.0"] pub DUPLICATE_MOD , suspicious , "file loaded as module multiple times" }
};
}
