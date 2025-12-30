// Generated macro for is_todo_unimplemented_stub (function)
macro_rules! Depcrate_usageis_todo_unimplemented_stub {
() => {
// Module: crate::usage
// Provides: {"is_todo_unimplemented_stub"}
// Dependencies: {}
# [doc = " Checks if the given expression is a stub, i.e., a `todo!()` or `unimplemented!()` expression,"] # [doc = " or a block whose last expression is a `todo!()` or `unimplemented!()`."] pub fn is_todo_unimplemented_stub (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { if let ExprKind :: Block (block , _) = expr . kind { if let Some (last_expr) = block . expr { return is_todo_unimplemented_macro (cx , last_expr) ; } return block . stmts . last () . is_some_and (| stmt | { if let hir :: StmtKind :: Expr (expr) | hir :: StmtKind :: Semi (expr) = stmt . kind { return is_todo_unimplemented_macro (cx , expr) ; } false }) ; } is_todo_unimplemented_macro (cx , expr) }
};
}
