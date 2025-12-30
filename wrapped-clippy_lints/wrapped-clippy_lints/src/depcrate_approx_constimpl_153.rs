// Generated macro for impl_153 (impl)
macro_rules! Depcrate_approx_constimpl_153 {
() => {
// Module: crate::approx_const
// Provides: {"impl_153"}
// Dependencies: {}
impl ApproxConstant { fn check_known_consts (& self , cx : & LateContext < '_ > , span : Span , s : symbol :: Symbol , module : & str) { let s = s . as_str () ; if let Ok (maybe_constant) = s . parse :: < f64 > () { for & (constant , name , min_digits , msrv) in & KNOWN_CONSTS { if is_approx_const (constant , s , maybe_constant , min_digits) && msrv . is_none_or (| msrv | self . msrv . meets (cx , msrv)) { span_lint_and_help (cx , APPROX_CONSTANT , span , format ! ("approximate value of `{module}::consts::{name}` found") , None , "consider using the constant directly" ,) ; return ; } } } } }
};
}
