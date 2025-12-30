// Generated macro for get_lint_and_message (function)
macro_rules! Depcrate_operators_float_cmpget_lint_and_message {
() => {
// Module: crate::operators::float_cmp
// Provides: {"get_lint_and_message"}
// Dependencies: {}
fn get_lint_and_message (is_local : bool , is_comparing_arrays : bool) -> (& 'static rustc_lint :: Lint , & 'static str) { if is_local { (FLOAT_CMP , if is_comparing_arrays { "strict comparison of `f32` or `f64` arrays" } else { "strict comparison of `f32` or `f64`" } ,) } else { (FLOAT_CMP_CONST , if is_comparing_arrays { "strict comparison of `f32` or `f64` constant arrays" } else { "strict comparison of `f32` or `f64` constant" } ,) } }
};
}
