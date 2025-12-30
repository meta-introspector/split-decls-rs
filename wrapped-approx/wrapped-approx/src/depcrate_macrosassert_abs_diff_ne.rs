// Generated macro for assert_abs_diff_ne (macro)
macro_rules! Depcrate_macrosassert_abs_diff_ne {
() => {
// Module: crate::macros
// Provides: {"assert_abs_diff_ne"}
// Dependencies: {}
# [doc = " An assertion that delegates to [`abs_diff_ne!`], and panics with a helpful error on failure."] # [macro_export (local_inner_macros)] macro_rules ! assert_abs_diff_ne { ($ given : expr , $ expected : expr $ (, $ opt : ident = $ val : expr) *) => { __assert_approx ! (abs_diff_ne , $ given , $ expected $ (, $ opt = $ val) *) } ; ($ given : expr , $ expected : expr $ (, $ opt : ident = $ val : expr) *,) => { __assert_approx ! (abs_diff_ne , $ given , $ expected $ (, $ opt = $ val) *) } ; }
};
}
