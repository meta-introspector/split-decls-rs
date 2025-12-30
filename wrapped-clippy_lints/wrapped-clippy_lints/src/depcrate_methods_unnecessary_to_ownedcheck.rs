// Generated macro for check (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedcheck {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"check"}
// Dependencies: {}
pub fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , method_name : Symbol , receiver : & 'tcx Expr < '_ > , args : & 'tcx [Expr < '_ >] , msrv : Msrv ,) { if let Some (method_parent_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) . opt_parent (cx) && args . is_empty () { if is_cloned_or_copied (cx , method_name , method_parent_id) { unnecessary_iter_cloned :: check (cx , expr , method_name , receiver) ; } else if is_to_owned_like (cx , expr , method_name , method_parent_id) { if check_split_call_arg (cx , expr , method_name , receiver) { return ; } if check_addr_of_expr (cx , expr , method_name , method_parent_id , receiver) { return ; } if check_into_iter_call_arg (cx , expr , method_name , receiver , msrv) { return ; } if check_string_from_utf8 (cx , expr , receiver) { return ; } check_other_call_arg (cx , expr , method_name , receiver) ; } } else { check_borrow_predicate (cx , expr) ; } }
};
}
