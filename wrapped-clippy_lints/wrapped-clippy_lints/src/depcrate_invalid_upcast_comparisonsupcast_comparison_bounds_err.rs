// Generated macro for upcast_comparison_bounds_err (function)
macro_rules! Depcrate_invalid_upcast_comparisonsupcast_comparison_bounds_err {
() => {
// Module: crate::invalid_upcast_comparisons
// Provides: {"upcast_comparison_bounds_err"}
// Dependencies: {}
fn upcast_comparison_bounds_err < 'tcx > (cx : & LateContext < 'tcx > , span : Span , rel : Rel , lhs_bounds : Option < (FullInt , FullInt) > , lhs : & 'tcx Expr < '_ > , rhs : & 'tcx Expr < '_ > , invert : bool ,) { if let Some ((lb , ub)) = lhs_bounds && let Some (norm_rhs_val) = ConstEvalCtxt :: new (cx) . eval_full_int (rhs) { if rel == Rel :: Eq || rel == Rel :: Ne { if norm_rhs_val < lb || norm_rhs_val > ub { err_upcast_comparison (cx , span , lhs , rel == Rel :: Ne) ; } } else if match rel { Rel :: Lt => { if invert { norm_rhs_val < lb } else { ub < norm_rhs_val } } , Rel :: Le => { if invert { norm_rhs_val <= lb } else { ub <= norm_rhs_val } } , Rel :: Eq | Rel :: Ne => unreachable ! () , } { err_upcast_comparison (cx , span , lhs , true) ; } else if match rel { Rel :: Lt => { if invert { norm_rhs_val >= ub } else { lb >= norm_rhs_val } } , Rel :: Le => { if invert { norm_rhs_val > ub } else { lb > norm_rhs_val } } , Rel :: Eq | Rel :: Ne => unreachable ! () , } { err_upcast_comparison (cx , span , lhs , false) ; } } }
};
}
