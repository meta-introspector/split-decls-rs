// Generated macro for comparison_to_const (function)
macro_rules! Depcrate_operators_const_comparisonscomparison_to_const {
() => {
// Module: crate::operators::const_comparisons
// Provides: {"comparison_to_const"}
// Dependencies: {}
fn comparison_to_const < 'tcx > (cx : & LateContext < 'tcx > , typeck : & 'tcx TypeckResults < 'tcx > , expr : & 'tcx Expr < 'tcx > ,) -> Option < (CmpOp , & 'tcx Expr < 'tcx > , & 'tcx Expr < 'tcx > , Constant , Ty < 'tcx >) > { if let ExprKind :: Binary (operator , left , right) = expr . kind && let Ok (cmp_op) = CmpOp :: try_from (operator . node) { let ecx = ConstEvalCtxt :: with_env (cx . tcx , cx . typing_env () , typeck) ; match (ecx . eval (left) , ecx . eval (right)) { (Some (_) , Some (_)) => None , (_ , Some (con)) => Some ((cmp_op , left , right , con , typeck . expr_ty (right))) , (Some (con) , _) => Some ((cmp_op . reverse () , right , left , con , typeck . expr_ty (left))) , _ => None , } } else { None } }
};
}
