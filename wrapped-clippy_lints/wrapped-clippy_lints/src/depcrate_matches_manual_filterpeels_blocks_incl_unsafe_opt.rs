// Generated macro for peels_blocks_incl_unsafe_opt (function)
macro_rules! Depcrate_matches_manual_filterpeels_blocks_incl_unsafe_opt {
() => {
// Module: crate::matches::manual_filter
// Provides: {"peels_blocks_incl_unsafe_opt"}
// Dependencies: {}
fn peels_blocks_incl_unsafe_opt < 'a > (expr : & 'a Expr < 'a >) -> Option < & 'a Expr < 'a > > { if let ExprKind :: Block (block , None) = expr . kind && block . stmts . is_empty () { return block . expr ; } None }
};
}
