// Generated macro for walk_filter_map_expr (function)
macro_rules! Depcrate_mut_visitwalk_filter_map_expr {
() => {
// Module: crate::mut_visit
// Provides: {"walk_filter_map_expr"}
// Dependencies: {}
pub fn walk_filter_map_expr < T : MutVisitor > (vis : & mut T , mut e : Box < Expr >) -> Option < Box < Expr > > { vis . visit_expr (& mut e) ; Some (e) }
};
}
