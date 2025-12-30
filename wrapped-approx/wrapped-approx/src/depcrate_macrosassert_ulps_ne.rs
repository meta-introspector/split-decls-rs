// Generated macro for assert_ulps_ne (macro)
macro_rules! Depcrate_macrosassert_ulps_ne {
() => {
// Module: crate::macros
// Provides: {"assert_ulps_ne"}
// Dependencies: {}
# [doc = " An assertion that delegates to [`ulps_ne!`], and panics with a helpful error on failure."] # [macro_export (local_inner_macros)] macro_rules ! assert_ulps_ne { ($ given : expr , $ expected : expr $ (, $ opt : ident = $ val : expr) *) => { __assert_approx ! (ulps_ne , $ given , $ expected $ (, $ opt = $ val) *) } ; ($ given : expr , $ expected : expr $ (, $ opt : ident = $ val : expr) *,) => { __assert_approx ! (ulps_ne , $ given , $ expected $ (, $ opt = $ val) *) } ; }
};
}
