// Generated macro for check_xor_swap (function)
macro_rules! Depcrate_swapcheck_xor_swap {
() => {
// Module: crate::swap
// Provides: {"check_xor_swap"}
// Dependencies: {}
# [doc = " Implementation of the xor case for `MANUAL_SWAP` lint."] fn check_xor_swap < 'tcx > (cx : & LateContext < 'tcx > , block : & 'tcx Block < 'tcx >) { for [s1 , s2 , s3] in block . stmts . array_windows :: < 3 > () { let ctxt = s1 . span . ctxt () ; if let Some ((lhs0 , rhs0)) = extract_sides_of_xor_assign (s1 , ctxt) && let Some ((lhs1 , rhs1)) = extract_sides_of_xor_assign (s2 , ctxt) && let Some ((lhs2 , rhs2)) = extract_sides_of_xor_assign (s3 , ctxt) && eq_expr_value (cx , lhs0 , rhs1) && eq_expr_value (cx , lhs2 , rhs1) && eq_expr_value (cx , lhs1 , rhs0) && eq_expr_value (cx , lhs1 , rhs2) && s2 . span . ctxt () == ctxt && s3 . span . ctxt () == ctxt { let span = s1 . span . to (s3 . span) ; generate_swap_warning (block , cx , lhs0 , rhs0 , rhs1 , rhs2 , span , true) ; } } }
};
}
