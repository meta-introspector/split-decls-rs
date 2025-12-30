// Generated macro for macro_2033 (macro)
macro_rules! Depcrate_escapemacro_2033 {
() => {
// Module: crate::escape
// Provides: {"macro_2033"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `Box<T>` where an unboxed `T` would"] # [doc = " work fine."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is an unnecessary allocation, and bad for"] # [doc = " performance. It is only necessary to allocate if you wish to move the box"] # [doc = " into something."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo(x: Box<u32>) {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn foo(x: u32) {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub BOXED_LOCAL , perf , "using `Box<T>` where unnecessary" }
};
}
