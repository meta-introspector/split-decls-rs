// Generated macro for macro_7632 (macro)
macro_rules! Depcrate_multi_assignmentsmacro_7632 {
() => {
// Module: crate::multi_assignments
// Provides: {"macro_7632"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for nested assignments."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " While this is in most cases already a type mismatch,"] # [doc = " the result of an assignment being `()` can throw off people coming from languages like python or C,"] # [doc = " where such assignments return a copy of the assigned value."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = "# let (a, b);"] # [doc = " a = b = 42;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = "# let (a, b);"] # [doc = " b = 42;"] # [doc = " a = b;"] # [doc = " ```"] # [clippy :: version = "1.65.0"] pub MULTI_ASSIGNMENTS , suspicious , "instead of using `a = b = c;` use `a = c; b = c;`" }
};
}
