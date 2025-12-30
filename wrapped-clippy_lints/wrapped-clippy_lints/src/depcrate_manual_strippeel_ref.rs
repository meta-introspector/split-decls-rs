// Generated macro for peel_ref (function)
macro_rules! Depcrate_manual_strippeel_ref {
() => {
// Module: crate::manual_strip
// Provides: {"peel_ref"}
// Dependencies: {}
fn peel_ref < 'a > (expr : & 'a Expr < '_ >) -> & 'a Expr < 'a > { if let ExprKind :: AddrOf (BorrowKind :: Ref , _ , unref) = & expr . kind { unref } else { expr } }
};
}
