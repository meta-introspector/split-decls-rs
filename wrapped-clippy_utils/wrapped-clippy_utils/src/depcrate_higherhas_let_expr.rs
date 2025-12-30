// Generated macro for has_let_expr (function)
macro_rules! Depcrate_higherhas_let_expr {
() => {
// Module: crate::higher
// Provides: {"has_let_expr"}
// Dependencies: {}
# [doc = " Checks that a condition doesn't have a `let` expression, to keep `If` and `While` from accepting"] # [doc = " `if let` and `while let`."] pub const fn has_let_expr < 'tcx > (cond : & 'tcx Expr < 'tcx >) -> bool { match & cond . kind { ExprKind :: Let (_) => true , ExprKind :: Binary (_ , lhs , rhs) => has_let_expr (lhs) || has_let_expr (rhs) , _ => false , } }
};
}
