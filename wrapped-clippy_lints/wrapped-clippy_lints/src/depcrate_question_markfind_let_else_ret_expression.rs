// Generated macro for find_let_else_ret_expression (function)
macro_rules! Depcrate_question_markfind_let_else_ret_expression {
() => {
// Module: crate::question_mark
// Provides: {"find_let_else_ret_expression"}
// Dependencies: {}
fn find_let_else_ret_expression < 'hir > (block : & 'hir Block < 'hir >) -> Option < & 'hir Expr < 'hir > > { if let Block { stmts : [] , expr : Some (els) , .. } = block { Some (els) } else if let [stmt] = block . stmts && let StmtKind :: Semi (expr) = stmt . kind && let ExprKind :: Ret (..) = expr . kind { Some (expr) } else { None } }
};
}
