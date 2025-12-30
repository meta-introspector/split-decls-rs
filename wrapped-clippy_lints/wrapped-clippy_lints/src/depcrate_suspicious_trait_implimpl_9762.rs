// Generated macro for impl_9762 (impl)
macro_rules! Depcrate_suspicious_trait_implimpl_9762 {
() => {
// Module: crate::suspicious_trait_impl
// Provides: {"impl_9762"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for SuspiciousImpl { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ >) { match expr . kind { hir :: ExprKind :: Binary (op , _ , _) => { check_expr_inner (cx , expr , op . node , op . span) ; } , hir :: ExprKind :: AssignOp (op , _ , _) => { check_expr_inner (cx , expr , op . node . into () , op . span) ; } , _ => { } , } } }
};
}
