// Generated macro for check_set (function)
macro_rules! Depcrate_regexcheck_set {
() => {
// Module: crate::regex
// Provides: {"check_set"}
// Dependencies: {}
fn check_set < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , utf8 : bool) { if let ExprKind :: AddrOf (BorrowKind :: Ref , _ , expr) = expr . kind && let ExprKind :: Array (exprs) = expr . kind { for expr in exprs { check_regex (cx , expr , utf8) ; } } }
};
}
