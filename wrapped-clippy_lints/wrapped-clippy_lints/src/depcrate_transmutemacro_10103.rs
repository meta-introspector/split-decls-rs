// Generated macro for macro_10103 (macro)
macro_rules! Depcrate_transmutemacro_10103 {
() => {
// Module: crate::transmute
// Provides: {"macro_10103"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for transmutes from a pointer to a reference."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This can always be rewritten with `&` and `*`."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " - `mem::transmute` in statics and constants is stable from Rust 1.46.0,"] # [doc = " while dereferencing raw pointer is not stable yet."] # [doc = " If you need to do this in those places,"] # [doc = " you would have to use `transmute` instead."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " unsafe {"] # [doc = "     let _: &T = std::mem::transmute(p); // where p: *const T"] # [doc = " }"] # [doc = ""] # [doc = " // can be written:"] # [doc = " let _: &T = &*p;"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub TRANSMUTE_PTR_TO_REF , complexity , "transmutes from a pointer to a reference type" }
};
}
