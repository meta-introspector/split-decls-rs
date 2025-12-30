// Generated macro for parse (function)
macro_rules! Depcrate_swapparse {
() => {
// Module: crate::swap
// Provides: {"parse"}
// Dependencies: {}
fn parse < 'a , 'hir > (stmt : & 'a Stmt < 'hir >) -> Option < (ExprOrIdent < 'hir > , & 'a Expr < 'hir >) > { if let StmtKind :: Semi (expr) = stmt . kind { if let ExprKind :: Assign (lhs , rhs , _) = expr . kind { return Some ((ExprOrIdent :: Expr (lhs) , rhs)) ; } } else if let StmtKind :: Let (expr) = stmt . kind && let Some (rhs) = expr . init && let PatKind :: Binding (_ , _ , ident_l , _) = expr . pat . kind { return Some ((ExprOrIdent :: Ident (ident_l) , rhs)) ; } None }
};
}
