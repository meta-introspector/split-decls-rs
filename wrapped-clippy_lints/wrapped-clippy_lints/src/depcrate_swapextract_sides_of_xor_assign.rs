// Generated macro for extract_sides_of_xor_assign (function)
macro_rules! Depcrate_swapextract_sides_of_xor_assign {
() => {
// Module: crate::swap
// Provides: {"extract_sides_of_xor_assign"}
// Dependencies: {}
# [doc = " Returns the lhs and rhs of an xor assignment statement."] fn extract_sides_of_xor_assign < 'a , 'hir > (stmt : & 'a Stmt < 'hir > , ctxt : SyntaxContext ,) -> Option < (& 'a Expr < 'hir > , & 'a Expr < 'hir >) > { if let StmtKind :: Semi (expr) = stmt . kind && let ExprKind :: AssignOp (Spanned { node : AssignOpKind :: BitXorAssign , .. } , lhs , rhs ,) = expr . kind && expr . span . ctxt () == ctxt { Some ((lhs , rhs)) } else { None } }
};
}
