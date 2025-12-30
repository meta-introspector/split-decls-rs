// Generated macro for macro_7216 (macro)
macro_rules! Depcrate_methodsmacro_7216 {
() => {
// Module: crate::methods
// Provides: {"macro_7216"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.skip(0)` on iterators."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This was likely intended to be `.skip(1)` to skip the first element, as `.skip(0)` does"] # [doc = " nothing. If not, the call should be removed."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let v = vec![1, 2, 3];"] # [doc = " let x = v.iter().skip(0).collect::<Vec<_>>();"] # [doc = " let y = v.iter().collect::<Vec<_>>();"] # [doc = " assert_eq!(x, y);"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub ITER_SKIP_ZERO , correctness , "disallows `.skip(0)`" }
};
}
