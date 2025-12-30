// Generated macro for impl_8594 (impl)
macro_rules! Depcrate_option_if_let_elseimpl_8594 {
() => {
// Module: crate::option_if_let_else
// Provides: {"impl_8594"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for OptionIfLetElse { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) { if expr . span . from_expansion () || is_in_const_context (cx) { return ; } let detection = detect_option_if_let_else (cx , expr) . or_else (| | detect_option_match (cx , expr)) ; if let Some (det) = detection { span_lint_and_sugg (cx , OPTION_IF_LET_ELSE , expr . span , format ! ("use Option::{} instead of an if let/else" , det . method_sugg) , "try" , format ! ("{}.{}({}, {})" , det . option , det . method_sugg , det . none_expr , det . some_expr) , Applicability :: MaybeIncorrect ,) ; } } }
};
}
