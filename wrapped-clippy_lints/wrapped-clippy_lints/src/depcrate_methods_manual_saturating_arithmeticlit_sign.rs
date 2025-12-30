// Generated macro for lit_sign (function)
macro_rules! Depcrate_methods_manual_saturating_arithmeticlit_sign {
() => {
// Module: crate::methods::manual_saturating_arithmetic
// Provides: {"lit_sign"}
// Dependencies: {}
fn lit_sign (expr : & hir :: Expr < '_ >) -> Option < Sign > { if let hir :: ExprKind :: Unary (hir :: UnOp :: Neg , inner) = & expr . kind { if let hir :: ExprKind :: Lit (..) = & inner . kind { return Some (Sign :: Neg) ; } } else if let hir :: ExprKind :: Lit (..) = & expr . kind { return Some (Sign :: Pos) ; } None }
};
}
