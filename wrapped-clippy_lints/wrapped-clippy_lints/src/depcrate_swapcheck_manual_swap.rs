// Generated macro for check_manual_swap (function)
macro_rules! Depcrate_swapcheck_manual_swap {
() => {
// Module: crate::swap
// Provides: {"check_manual_swap"}
// Dependencies: {}
# [doc = " Implementation of the `MANUAL_SWAP` lint."] fn check_manual_swap < 'tcx > (cx : & LateContext < 'tcx > , block : & 'tcx Block < 'tcx >) { if is_in_const_context (cx) { return ; } for [s1 , s2 , s3] in block . stmts . array_windows :: < 3 > () { if let StmtKind :: Let (tmp) = s1 . kind && let Some (tmp_init) = tmp . init && let PatKind :: Binding (.. , ident , None) = tmp . pat . kind && let StmtKind :: Semi (first) = s2 . kind && let ExprKind :: Assign (lhs1 , rhs1 , _) = first . kind && let StmtKind :: Semi (second) = s3 . kind && let ExprKind :: Assign (lhs2 , rhs2 , _) = second . kind && let ExprKind :: Path (QPath :: Resolved (None , rhs2_path)) = rhs2 . kind && rhs2_path . segments . len () == 1 && ident . name == rhs2_path . segments [0] . ident . name && eq_expr_value (cx , tmp_init , lhs1) && eq_expr_value (cx , rhs1 , lhs2) && let ctxt = s1 . span . ctxt () && s2 . span . ctxt () == ctxt && s3 . span . ctxt () == ctxt && first . span . ctxt () == ctxt && second . span . ctxt () == ctxt { let span = s1 . span . to (s3 . span) ; generate_swap_warning (block , cx , lhs1 , lhs2 , rhs1 , rhs2 , span , false) ; } } }
};
}
