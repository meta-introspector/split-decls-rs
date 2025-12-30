// Generated macro for check_assign (function)
macro_rules! Depcrate_let_if_seqcheck_assign {
() => {
// Module: crate::let_if_seq
// Provides: {"check_assign"}
// Dependencies: {}
fn check_assign < 'tcx > (cx : & LateContext < 'tcx > , decl : hir :: HirId , block : & 'tcx hir :: Block < '_ > ,) -> Option < & 'tcx hir :: Expr < 'tcx > > { if block . expr . is_none () && let Some (expr) = block . stmts . iter () . last () && let hir :: StmtKind :: Semi (expr) = expr . kind && let hir :: ExprKind :: Assign (var , value , _) = expr . kind && var . res_local_id () == Some (decl) { if block . stmts . iter () . take (block . stmts . len () - 1) . any (| stmt | is_local_used (cx , stmt , decl)) { None } else { Some (value) } } else { None } }
};
}
