// Generated macro for assert_relative_ne (macro)
macro_rules! Depcrate_macrosassert_relative_ne {
() => {
// Module: crate::macros
// Provides: {"assert_relative_ne"}
// Dependencies: {}
# [doc = " An assertion that delegates to [`relative_ne!`], and panics with a helpful error on failure."] # [macro_export (local_inner_macros)] macro_rules ! assert_relative_ne { ($ given : expr , $ expected : expr $ (, $ opt : ident = $ val : expr) *) => { __assert_approx ! (relative_ne , $ given , $ expected $ (, $ opt = $ val) *) } ; ($ given : expr , $ expected : expr $ (, $ opt : ident = $ val : expr) *,) => { __assert_approx ! (relative_ne , $ given , $ expected $ (, $ opt = $ val) *) } ; }
};
}
