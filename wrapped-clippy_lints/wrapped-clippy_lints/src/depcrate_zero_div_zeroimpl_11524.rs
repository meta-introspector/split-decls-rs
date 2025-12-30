// Generated macro for impl_11524 (impl)
macro_rules! Depcrate_zero_div_zeroimpl_11524 {
() => {
// Module: crate::zero_div_zero
// Provides: {"impl_11524"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for ZeroDiv { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: Binary (ref op , left , right) = expr . kind && op . node == BinOpKind :: Div && let ecx = ConstEvalCtxt :: new (cx) && let ctxt = expr . span . ctxt () && let Some (lhs_value) = ecx . eval_local (left , ctxt) && let Some (rhs_value) = ecx . eval_local (right , ctxt) && (Constant :: F32 (0.0) == lhs_value || Constant :: F64 (0.0) == lhs_value) && (Constant :: F32 (0.0) == rhs_value || Constant :: F64 (0.0) == rhs_value) { let float_type = match (lhs_value , rhs_value) { (Constant :: F64 (_) , _) | (_ , Constant :: F64 (_)) => "f64" , _ => "f32" , } ; span_lint_and_help (cx , ZERO_DIVIDED_BY_ZERO , expr . span , "constant division of `0.0` with `0.0` will always result in NaN" , None , format ! ("consider using `{float_type}::NAN` if you would like a constant representing NaN" ,) ,) ; } } }
};
}
