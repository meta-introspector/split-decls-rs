// Generated macro for macro_10264 (macro)
macro_rules! Depcrate_typesmacro_10264 {
() => {
// Module: crate::types
// Provides: {"macro_10264"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `Box<T>` where T is a collection such as Vec anywhere in the code."] # [doc = " Check the [Box documentation](https://doc.rust-lang.org/std/boxed/index.html) for more information."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Collections already keeps their contents in a separate area on"] # [doc = " the heap. So if you `Box` them, you just add another level of indirection"] # [doc = " without any benefit whatsoever."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " struct X {"] # [doc = "     values: Box<Vec<Foo>>,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Better:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " struct X {"] # [doc = "     values: Vec<Foo>,"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.57.0"] pub BOX_COLLECTION , perf , "usage of `Box<Vec<T>>`, vector elements are already on the heap" }
};
}
