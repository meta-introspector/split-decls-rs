// Generated macro for is_unit_function (function)
macro_rules! Depcrate_map_unit_fnis_unit_function {
() => {
// Module: crate::map_unit_fn
// Provides: {"is_unit_function"}
// Dependencies: {}
fn is_unit_function (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> bool { let ty = cx . typeck_results () . expr_ty (expr) ; if let ty :: FnDef (id , _) = * ty . kind () && let Some (fn_type) = cx . tcx . fn_sig (id) . instantiate_identity () . no_bound_vars () { return is_unit_type (fn_type . output ()) ; } false }
};
}
