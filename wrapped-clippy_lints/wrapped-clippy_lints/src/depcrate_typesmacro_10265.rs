// Generated macro for macro_10265 (macro)
macro_rules! Depcrate_typesmacro_10265 {
() => {
// Module: crate::types
// Provides: {"macro_10265"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `Vec<Box<T>>` where T: Sized anywhere in the code."] # [doc = " Check the [Box documentation](https://doc.rust-lang.org/std/boxed/index.html) for more information."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `Vec` already keeps its contents in a separate area on"] # [doc = " the heap. So if you `Box` its contents, you just add another level of indirection."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct X {"] # [doc = "     values: Vec<Box<i32>>,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Better:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " struct X {"] # [doc = "     values: Vec<i32>,"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.33.0"] pub VEC_BOX , complexity , "usage of `Vec<Box<T>>` where T: Sized, vector elements are already on the heap" }
};
}
