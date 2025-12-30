// Generated macro for macro_7054 (macro)
macro_rules! Depcrate_methodsmacro_7054 {
() => {
// Module: crate::methods
// Provides: {"macro_7054"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Finds occurrences of `Vec::resize(0, an_int)`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is probably an argument inversion mistake."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " vec![1, 2, 3, 4, 5].resize(0, 5)"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " vec![1, 2, 3, 4, 5].clear()"] # [doc = " ```"] # [clippy :: version = "1.46.0"] pub VEC_RESIZE_TO_ZERO , correctness , "emptying a vector with `resize(0, an_int)` instead of `clear()` is probably an argument inversion mistake" }
};
}
