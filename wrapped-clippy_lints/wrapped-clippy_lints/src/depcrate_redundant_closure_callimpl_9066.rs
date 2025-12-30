// Generated macro for impl_9066 (impl)
macro_rules! Depcrate_redundant_closure_callimpl_9066 {
() => {
// Module: crate::redundant_closure_call
// Provides: {"impl_9066"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for ReturnVisitor { type Result = ControlFlow < () > ; fn visit_expr (& mut self , ex : & 'tcx hir :: Expr < 'tcx >) -> ControlFlow < () > { if let ExprKind :: Ret (_) | ExprKind :: Match (.. , hir :: MatchSource :: TryDesugar (_)) = ex . kind { return ControlFlow :: Break (()) ; } hir_visit :: walk_expr (self , ex) } }
};
}
