// Generated macro for macro_11002 (macro)
macro_rules! Depcrate_vec_init_then_pushmacro_11002 {
() => {
// Module: crate::vec_init_then_push
// Provides: {"macro_11002"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `push` immediately after creating a new `Vec`."] # [doc = ""] # [doc = " If the `Vec` is created using `with_capacity` this will only lint if the capacity is a"] # [doc = " constant and the number of pushes is greater than or equal to the initial capacity."] # [doc = ""] # [doc = " If the `Vec` is extended after the initial sequence of pushes and it was default initialized"] # [doc = " then this will only lint after there were at least four pushes. This number may change in"] # [doc = " the future."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `vec![]` macro is both more performant and easier to read than"] # [doc = " multiple `push` calls."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut v = Vec::new();"] # [doc = " v.push(0);"] # [doc = " v.push(1);"] # [doc = " v.push(2);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let v = vec![0, 1, 2];"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub VEC_INIT_THEN_PUSH , perf , "`push` immediately after `Vec` creation" }
};
}
