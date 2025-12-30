// Generated macro for is_inside_let_else (function)
macro_rules! Depcrateis_inside_let_else {
() => {
// Module: crate
// Provides: {"is_inside_let_else"}
// Dependencies: {}
# [doc = " Checks if the given expression is a part of `let else`"] # [doc = " returns `true` for both the `init` and the `else` part"] pub fn is_inside_let_else (tcx : TyCtxt < '_ > , expr : & Expr < '_ >) -> bool { let mut child_id = expr . hir_id ; for (parent_id , node) in tcx . hir_parent_iter (child_id) { if let Node :: LetStmt (LetStmt { init : Some (init) , els : Some (els) , .. }) = node && (init . hir_id == child_id || els . hir_id == child_id) { return true ; } child_id = parent_id ; } false }
};
}
