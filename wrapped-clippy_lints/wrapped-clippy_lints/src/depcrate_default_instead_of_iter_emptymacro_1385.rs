// Generated macro for macro_1385 (macro)
macro_rules! Depcrate_default_instead_of_iter_emptymacro_1385 {
() => {
// Module: crate::default_instead_of_iter_empty
// Provides: {"macro_1385"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " It checks for `std::iter::Empty::default()` and suggests replacing it with"] # [doc = " `std::iter::empty()`."] # [doc = " ### Why is this bad?"] # [doc = " `std::iter::empty()` is the more idiomatic way."] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let _ = std::iter::Empty::<usize>::default();"] # [doc = " let iter: std::iter::Empty<usize> = std::iter::Empty::default();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let _ = std::iter::empty::<usize>();"] # [doc = " let iter: std::iter::Empty<usize> = std::iter::empty();"] # [doc = " ```"] # [clippy :: version = "1.64.0"] pub DEFAULT_INSTEAD_OF_ITER_EMPTY , style , "check `std::iter::Empty::default()` and replace with `std::iter::empty()`" }
};
}
