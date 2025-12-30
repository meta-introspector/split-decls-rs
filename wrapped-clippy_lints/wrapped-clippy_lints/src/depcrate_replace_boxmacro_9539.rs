// Generated macro for macro_9539 (macro)
macro_rules! Depcrate_replace_boxmacro_9539 {
() => {
// Module: crate::replace_box
// Provides: {"macro_9539"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects assignments of `Default::default()` or `Box::new(value)`"] # [doc = " to a place of type `Box<T>`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This incurs an extra heap allocation compared to assigning the boxed"] # [doc = " storage."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut b = Box::new(1u32);"] # [doc = " b = Default::default();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let mut b = Box::new(1u32);"] # [doc = " *b = Default::default();"] # [doc = " ```"] # [clippy :: version = "1.92.0"] pub REPLACE_BOX , perf , "assigning a newly created box to `Box<T>` is inefficient" }
};
}
