// Generated macro for is_map_to_option (function)
macro_rules! Depcrate_methods_map_flattenis_map_to_option {
() => {
// Module: crate::methods::map_flatten
// Provides: {"is_map_to_option"}
// Dependencies: {}
fn is_map_to_option (cx : & LateContext < '_ > , map_arg : & Expr < '_ >) -> bool { let map_closure_ty = cx . typeck_results () . expr_ty (map_arg) ; match map_closure_ty . kind () { ty :: Closure (_ , _) | ty :: FnDef (_ , _) | ty :: FnPtr (..) => { let map_closure_sig = match map_closure_ty . kind () { ty :: Closure (_ , args) => args . as_closure () . sig () , _ => map_closure_ty . fn_sig (cx . tcx) , } ; let map_closure_return_ty = cx . tcx . instantiate_bound_regions_with_erased (map_closure_sig . output ()) ; map_closure_return_ty . is_diag_item (cx , sym :: Option) } , _ => false , } }
};
}
