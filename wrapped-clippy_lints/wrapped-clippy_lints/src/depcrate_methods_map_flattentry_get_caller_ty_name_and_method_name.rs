// Generated macro for try_get_caller_ty_name_and_method_name (function)
macro_rules! Depcrate_methods_map_flattentry_get_caller_ty_name_and_method_name {
() => {
// Module: crate::methods::map_flatten
// Provides: {"try_get_caller_ty_name_and_method_name"}
// Dependencies: {}
fn try_get_caller_ty_name_and_method_name (cx : & LateContext < '_ > , expr : & Expr < '_ > , caller_expr : & Expr < '_ > , map_arg : & Expr < '_ > ,) -> Option < (& 'static str , & 'static str) > { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) { if is_map_to_option (cx , map_arg) { Some (("Iterator" , "filter_map")) } else { Some (("Iterator" , "flat_map")) } } else { if let ty :: Adt (adt , _) = cx . typeck_results () . expr_ty (caller_expr) . kind () { match cx . tcx . get_diagnostic_name (adt . did ()) { Some (sym :: Option) => return Some (("Option" , "and_then")) , Some (sym :: Result) => return Some (("Result" , "and_then")) , _ => { } , } } None } }
};
}
