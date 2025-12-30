// Generated macro for check_for_mutability (function)
macro_rules! Depcrate_loops_mut_range_boundcheck_for_mutability {
() => {
// Module: crate::loops::mut_range_bound
// Provides: {"check_for_mutability"}
// Dependencies: {}
fn check_for_mutability (cx : & LateContext < '_ > , bound : & Expr < '_ >) -> Option < HirId > { if let Some (hir_id) = bound . res_local_id () && let Node :: Pat (pat) = cx . tcx . hir_node (hir_id) && let PatKind :: Binding (BindingMode :: MUT , ..) = pat . kind { return Some (hir_id) ; } None }
};
}
