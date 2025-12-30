// Generated macro for is_filter_some_map_unwrap (function)
macro_rules! Depcrate_methods_filter_mapis_filter_some_map_unwrap {
() => {
// Module: crate::methods::filter_map
// Provides: {"is_filter_some_map_unwrap"}
// Dependencies: {}
# [doc = " is `filter(|x| x.is_some()).map(|x| x.unwrap())`"] fn is_filter_some_map_unwrap (cx : & LateContext < '_ > , expr : & Expr < '_ > , filter_recv : & Expr < '_ > , filter_arg : & Expr < '_ > , map_arg : & Expr < '_ > ,) -> bool { let iterator = cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) ; let option = cx . typeck_results () . expr_ty (filter_recv) . is_diag_item (cx , sym :: Option) ; (iterator || option) && is_option_filter_map (cx , filter_arg , map_arg) }
};
}
