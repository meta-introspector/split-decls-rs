// Generated macro for upcast_comparison_bounds_err (function)
macro_rules! Depcrate_operators_invalid_upcast_comparisonsupcast_comparison_bounds_err {
() => {
// Module: crate::operators::invalid_upcast_comparisons
// Provides: {"upcast_comparison_bounds_err"}
// Dependencies: {}
fn upcast_comparison_bounds_err < 'tcx > (cx : & LateContext < 'tcx > , span : Span , rel : Rel , lhs_bounds : Option < (FullInt , FullInt) > , lhs : & 'tcx Expr < '_ > , rhs : & 'tcx Expr < '_ > , invert : bool ,) { if let Some ((lb , ub)) = lhs_bounds && let Some (norm_rhs_val) = ConstEvalCtxt :: new (cx) . eval_full_int (rhs , span . ctxt ()) { match rel { Rel :: Eq => { if norm_rhs_val < lb || ub < norm_rhs_val { err_upcast_comparison (cx , span , lhs , false) ; } } , Rel :: Ne => { if norm_rhs_val < lb || ub < norm_rhs_val { err_upcast_comparison (cx , span , lhs , true) ; } } , Rel :: Lt => { if (invert && norm_rhs_val < lb) || (! invert && ub < norm_rhs_val) { err_upcast_comparison (cx , span , lhs , true) ; } else if (! invert && norm_rhs_val <= lb) || (invert && ub <= norm_rhs_val) { err_upcast_comparison (cx , span , lhs , false) ; } } , Rel :: Le => { if (invert && norm_rhs_val <= lb) || (! invert && ub <= norm_rhs_val) { err_upcast_comparison (cx , span , lhs , true) ; } else if (! invert && norm_rhs_val < lb) || (invert && ub < norm_rhs_val) { err_upcast_comparison (cx , span , lhs , false) ; } } , } } }
};
}
