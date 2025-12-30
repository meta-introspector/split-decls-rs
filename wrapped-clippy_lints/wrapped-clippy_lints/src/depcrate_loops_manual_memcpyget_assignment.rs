// Generated macro for get_assignment (function)
macro_rules! Depcrate_loops_manual_memcpyget_assignment {
() => {
// Module: crate::loops::manual_memcpy
// Provides: {"get_assignment"}
// Dependencies: {}
fn get_assignment < 'tcx > (e : & 'tcx Expr < 'tcx >) -> Option < (& 'tcx Expr < 'tcx > , & 'tcx Expr < 'tcx >) > { if let ExprKind :: Assign (lhs , rhs , _) = e . kind { Some ((lhs , rhs)) } else { None } }
};
}
