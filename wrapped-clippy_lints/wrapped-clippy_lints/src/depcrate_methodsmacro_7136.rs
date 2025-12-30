// Generated macro for macro_7136 (macro)
macro_rules! Depcrate_methodsmacro_7136 {
() => {
// Module: crate::methods
// Provides: {"macro_7136"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calling `.step_by(0)` on iterators which panics."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This very much looks like an oversight. Use `panic!()` instead if you"] # [doc = " actually intend to panic."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,should_panic"] # [doc = " for x in (0..100).step_by(0) {"] # [doc = "     //.."] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ITERATOR_STEP_BY_ZERO , correctness , "using `Iterator::step_by(0)`, which will panic at runtime" }
};
}
