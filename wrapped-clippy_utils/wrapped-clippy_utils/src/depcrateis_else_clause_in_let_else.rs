// Generated macro for is_else_clause_in_let_else (function)
macro_rules! Depcrateis_else_clause_in_let_else {
() => {
// Module: crate
// Provides: {"is_else_clause_in_let_else"}
// Dependencies: {}
# [doc = " Checks if the given expression is the else clause of a `let else` expression"] pub fn is_else_clause_in_let_else (tcx : TyCtxt < '_ > , expr : & Expr < '_ >) -> bool { let mut child_id = expr . hir_id ; for (parent_id , node) in tcx . hir_parent_iter (child_id) { if let Node :: LetStmt (LetStmt { els : Some (els) , .. }) = node && els . hir_id == child_id { return true ; } child_id = parent_id ; } false }
};
}
