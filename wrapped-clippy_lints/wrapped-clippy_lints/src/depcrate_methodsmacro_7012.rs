// Generated macro for macro_7012 (macro)
macro_rules! Depcrate_methodsmacro_7012 {
() => {
// Module: crate::methods
// Provides: {"macro_7012"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `iter().next()` on a Slice or an Array"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " These can be shortened into `.get()`"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let a = [1, 2, 3];"] # [doc = " # let b = vec![1, 2, 3];"] # [doc = " a[2..].iter().next();"] # [doc = " b.iter().next();"] # [doc = " ```"] # [doc = " should be written as:"] # [doc = " ```no_run"] # [doc = " # let a = [1, 2, 3];"] # [doc = " # let b = vec![1, 2, 3];"] # [doc = " a.get(2);"] # [doc = " b.get(0);"] # [doc = " ```"] # [clippy :: version = "1.46.0"] pub ITER_NEXT_SLICE , style , "using `.iter().next()` on a sliced array, which can be shortened to just `.get()`" }
};
}
