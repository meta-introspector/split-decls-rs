// Generated macro for check_result (macro)
macro_rules! Depcrate_rasterizercheck_result {
() => {
// Module: crate::rasterizer
// Provides: {"check_result"}
// Dependencies: {}
macro_rules ! check_result { ($ e : expr) => { let result = $ e ; # [allow (clippy :: question_mark)] if result . is_err () { return result ; } } ; }
};
}
