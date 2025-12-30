// Generated macro for impl_10842 (impl)
macro_rules! Depcrate_unwrapimpl_10842 {
() => {
// Module: crate::unwrap
// Provides: {"impl_10842"}
// Dependencies: {}
impl < 'tcx > UnwrappableVariablesVisitor < '_ , 'tcx > { fn visit_branch (& mut self , if_expr : & 'tcx Expr < '_ > , cond : & 'tcx Expr < '_ > , branch : & 'tcx Expr < '_ > , else_branch : bool ,) { let prev_len = self . unwrappables . len () ; for unwrap_info in collect_unwrap_info (self . cx , if_expr , cond , branch , else_branch , true) { let mut delegate = MutationVisitor { is_mutated : false , local_id : unwrap_info . local_id , tcx : self . cx . tcx , } ; let vis = ExprUseVisitor :: for_clippy (self . cx , cond . hir_id . owner . def_id , & mut delegate) ; vis . walk_expr (cond) . into_ok () ; vis . walk_expr (branch) . into_ok () ; if delegate . is_mutated { continue ; } self . unwrappables . push (unwrap_info) ; } walk_expr (self , branch) ; self . unwrappables . truncate (prev_len) ; } }
};
}
