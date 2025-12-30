// Generated macro for impl_9845 (impl)
macro_rules! Depcrate_temporary_assignmentimpl_9845 {
() => {
// Module: crate::temporary_assignment
// Provides: {"impl_9845"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for TemporaryAssignment { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let ExprKind :: Assign (target , ..) = & expr . kind { let mut base = target ; while let ExprKind :: Field (f , _) | ExprKind :: Index (f , _ , _) = & base . kind { base = f ; } if is_temporary (base) && ! is_adjusted (cx , base) { span_lint (cx , TEMPORARY_ASSIGNMENT , expr . span , "assignment to temporary") ; } } } }
};
}
