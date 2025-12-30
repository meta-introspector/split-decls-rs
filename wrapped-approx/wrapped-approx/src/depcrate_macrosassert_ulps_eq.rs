// Generated macro for assert_ulps_eq (macro)
macro_rules! Depcrate_macrosassert_ulps_eq {
() => {
// Module: crate::macros
// Provides: {"assert_ulps_eq"}
// Dependencies: {}
# [doc = " An assertion that delegates to [`ulps_eq!`], and panics with a helpful error on failure."] # [macro_export (local_inner_macros)] macro_rules ! assert_ulps_eq { ($ given : expr , $ expected : expr $ (, $ opt : ident = $ val : expr) *) => { __assert_approx ! (ulps_eq , $ given , $ expected $ (, $ opt = $ val) *) } ; ($ given : expr , $ expected : expr $ (, $ opt : ident = $ val : expr) *,) => { __assert_approx ! (ulps_eq , $ given , $ expected $ (, $ opt = $ val) *) } ; }
};
}
