// Generated macro for is_expr_used_or_unified (function)
macro_rules! Depcrateis_expr_used_or_unified {
() => {
// Module: crate
// Provides: {"is_expr_used_or_unified"}
// Dependencies: {}
# [doc = " Checks if the result of an expression is used, or it's type is unified with another branch."] pub fn is_expr_used_or_unified (tcx : TyCtxt < '_ > , expr : & Expr < '_ >) -> bool { ! matches ! (get_expr_use_or_unification_node (tcx , expr) , None | Some ((Node :: Stmt (Stmt { kind : StmtKind :: Expr (_) | StmtKind :: Semi (_) | StmtKind :: Let (LetStmt { pat : Pat { kind : PatKind :: Wild , .. } , .. }) , .. }) , _))) }
};
}
