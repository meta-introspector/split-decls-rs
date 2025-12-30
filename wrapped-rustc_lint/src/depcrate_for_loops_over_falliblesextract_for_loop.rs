// Generated macro for extract_for_loop (function)
macro_rules! Depcrate_for_loops_over_falliblesextract_for_loop {
() => {
// Module: crate::for_loops_over_fallibles
// Provides: {"extract_for_loop"}
// Dependencies: {}
fn extract_for_loop < 'tcx > (expr : & Expr < 'tcx >) -> Option < (& 'tcx Pat < 'tcx > , & 'tcx Expr < 'tcx >) > { if let hir :: ExprKind :: DropTemps (e) = expr . kind && let hir :: ExprKind :: Match (iterexpr , [arm] , hir :: MatchSource :: ForLoopDesugar) = e . kind && let hir :: ExprKind :: Call (_ , [arg]) = iterexpr . kind && let hir :: ExprKind :: Loop (block , ..) = arm . body . kind && let [stmt] = block . stmts && let hir :: StmtKind :: Expr (e) = stmt . kind && let hir :: ExprKind :: Match (_ , [_ , some_arm] , _) = e . kind && let hir :: PatKind :: Struct (_ , [field] , _) = some_arm . pat . kind { Some ((field . pat , arg)) } else { None } }
};
}
