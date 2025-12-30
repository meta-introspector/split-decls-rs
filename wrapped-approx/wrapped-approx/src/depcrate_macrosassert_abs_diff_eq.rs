// Generated macro for assert_abs_diff_eq (macro)
macro_rules! Depcrate_macrosassert_abs_diff_eq {
() => {
// Module: crate::macros
// Provides: {"assert_abs_diff_eq"}
// Dependencies: {}
# [doc = " An assertion that delegates to [`abs_diff_eq!`], and panics with a helpful error on failure."] # [macro_export (local_inner_macros)] macro_rules ! assert_abs_diff_eq { ($ given : expr , $ expected : expr $ (, $ opt : ident = $ val : expr) *) => { __assert_approx ! (abs_diff_eq , $ given , $ expected $ (, $ opt = $ val) *) } ; ($ given : expr , $ expected : expr $ (, $ opt : ident = $ val : expr) *,) => { __assert_approx ! (abs_diff_eq , $ given , $ expected $ (, $ opt = $ val) *) } ; }
};
}
