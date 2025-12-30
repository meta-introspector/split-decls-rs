// Generated macro for impl_8639 (impl)
macro_rules! Depcrate_panicking_overflow_checksimpl_8639 {
() => {
// Module: crate::panicking_overflow_checks
// Provides: {"impl_8639"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for PanickingOverflowChecks { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: Binary (op , lhs , rhs) = expr . kind && let (lt , gt) = match op . node { BinOpKind :: Lt => (lhs , rhs) , BinOpKind :: Gt => (rhs , lhs) , _ => return , } && let ctxt = expr . span . ctxt () && let (op_lhs , op_rhs , other , commutative) = match (& lt . kind , & gt . kind) { (& ExprKind :: Binary (op , lhs , rhs) , _) if op . node == BinOpKind :: Add && ctxt == lt . span . ctxt () => { (lhs , rhs , gt , true) } , (_ , & ExprKind :: Binary (op , lhs , rhs)) if op . node == BinOpKind :: Sub && ctxt == gt . span . ctxt () => { (lhs , rhs , lt , false) } , _ => return , } && let typeck = cx . typeck_results () && let ty = typeck . expr_ty (op_lhs) && matches ! (ty . kind () , ty :: Uint (_)) && ty == typeck . expr_ty (op_rhs) && ty == typeck . expr_ty (other) && ! expr . span . in_external_macro (cx . tcx . sess . source_map ()) && (eq_expr_value (cx , op_lhs , other) || (commutative && eq_expr_value (cx , op_rhs , other))) { span_lint (cx , PANICKING_OVERFLOW_CHECKS , expr . span , "you are trying to use classic C overflow conditions that will fail in Rust" ,) ; } } }
};
}
