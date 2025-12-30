// Generated macro for assert_relative_eq (macro)
macro_rules! Depcrate_macrosassert_relative_eq {
() => {
// Module: crate::macros
// Provides: {"assert_relative_eq"}
// Dependencies: {}
# [doc = " An assertion that delegates to [`relative_eq!`], and panics with a helpful error on failure."] # [macro_export (local_inner_macros)] macro_rules ! assert_relative_eq { ($ given : expr , $ expected : expr $ (, $ opt : ident = $ val : expr) *) => { __assert_approx ! (relative_eq , $ given , $ expected $ (, $ opt = $ val) *) } ; ($ given : expr , $ expected : expr $ (, $ opt : ident = $ val : expr) *,) => { __assert_approx ! (relative_eq , $ given , $ expected $ (, $ opt = $ val) *) } ; }
};
}
