// Generated macro for impl_7909 (impl)
macro_rules! Depcrate_needless_late_initimpl_7909 {
() => {
// Module: crate::needless_late_init
// Provides: {"impl_7909"}
// Dependencies: {}
impl LocalAssign { fn from_expr (expr : & Expr < '_ > , span : Span) -> Option < Self > { if expr . span . from_expansion () { return None ; } if let ExprKind :: Assign (lhs , rhs , _) = expr . kind { if lhs . span . from_expansion () { return None ; } Some (Self { lhs_id : lhs . res_local_id () ? , rhs_span : rhs . span . source_callsite () , span , }) } else { None } } fn new < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , binding_id : HirId) -> Option < LocalAssign > { let assign = match expr . kind { ExprKind :: Block (Block { expr : Some (expr) , .. } , _) => Self :: from_expr (expr , expr . span) , ExprKind :: Block (block , _) => { if let Some ((last , other_stmts)) = block . stmts . split_last () && let StmtKind :: Expr (expr) | StmtKind :: Semi (expr) = last . kind && let assign = Self :: from_expr (expr , last . span) ? && assign . lhs_id == binding_id && other_stmts . iter () . all (| stmt | ! contains_assign_expr (cx , stmt)) { Some (assign) } else { None } } , ExprKind :: Assign (..) => Self :: from_expr (expr , expr . span) , _ => None , } ? ; if assign . lhs_id == binding_id { Some (assign) } else { None } } }
};
}
