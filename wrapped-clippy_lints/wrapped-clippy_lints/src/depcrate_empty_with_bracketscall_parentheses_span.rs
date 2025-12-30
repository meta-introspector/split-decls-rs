// Generated macro for call_parentheses_span (function)
macro_rules! Depcrate_empty_with_bracketscall_parentheses_span {
() => {
// Module: crate::empty_with_brackets
// Provides: {"call_parentheses_span"}
// Dependencies: {}
fn call_parentheses_span (tcx : TyCtxt < '_ > , expr : & Expr < '_ >) -> Option < Span > { if let Node :: Expr (parent) = tcx . parent_hir_node (expr . hir_id) && let ExprKind :: Call (callee , ..) = parent . kind && callee . hir_id == expr . hir_id { Some (parent . span . with_lo (expr . span . hi ())) } else { None } }
};
}
