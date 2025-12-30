// Generated macro for impl_2220 (impl)
macro_rules! Depcrate_floating_point_arithmeticimpl_2220 {
() => {
// Module: crate::floating_point_arithmetic
// Provides: {"impl_2220"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for FloatingPointArithmetic { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if is_in_const_context (cx) { return ; } if let ExprKind :: MethodCall (path , receiver , args , _) = & expr . kind { let recv_ty = cx . typeck_results () . expr_ty (receiver) ; if recv_ty . is_floating_point () && ! is_no_std_crate (cx) && is_inherent_method_call (cx , expr) { match path . ident . name { sym :: ln => check_ln1p (cx , expr , receiver) , sym :: log => check_log_base (cx , expr , receiver , args) , sym :: powf => check_powf (cx , expr , receiver , args) , sym :: powi => check_powi (cx , expr , receiver , args) , sym :: sqrt => check_hypot (cx , expr , receiver) , _ => { } , } } } else { if ! is_no_std_crate (cx) { check_expm1 (cx , expr) ; check_mul_add (cx , expr) ; check_custom_abs (cx , expr) ; check_log_division (cx , expr) ; } check_radians (cx , expr) ; } } }
};
}
