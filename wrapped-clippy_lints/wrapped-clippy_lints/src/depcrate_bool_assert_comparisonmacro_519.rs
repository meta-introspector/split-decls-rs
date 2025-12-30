// Generated macro for macro_519 (macro)
macro_rules! Depcrate_bool_assert_comparisonmacro_519 {
() => {
// Module: crate::bool_assert_comparison
// Provides: {"macro_519"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This lint warns about boolean comparisons in assert-like macros."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is shorter to use the equivalent."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " assert_eq!(\"a\".is_empty(), false);"] # [doc = " assert_ne!(\"a\".is_empty(), true);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " assert!(!\"a\".is_empty());"] # [doc = " ```"] # [clippy :: version = "1.53.0"] pub BOOL_ASSERT_COMPARISON , style , "Using a boolean as comparison value in an assert_* macro when there is no need" }
};
}
