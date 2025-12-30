// Generated macro for methods_pattern (function)
macro_rules! Depcrate_methods_range_zip_with_lenmethods_pattern {
() => {
// Module: crate::methods::range_zip_with_len
// Provides: {"methods_pattern"}
// Dependencies: {}
# [doc = " If `expr` is the receiver of an `Iterator` method which consumes the iterator elements and feed"] # [doc = " them to a closure, return the pattern of the closure."] fn methods_pattern < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> Option < & 'tcx Pat < 'tcx > > { if let Some (parent_expr) = get_parent_expr (cx , expr) && cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && let ExprKind :: MethodCall (method , recv , [arg] , _) = parent_expr . kind && recv . hir_id == expr . hir_id && matches ! (method . ident . name , sym :: all | sym :: any | sym :: filter_map | sym :: find_map | sym :: flat_map | sym :: for_each | sym :: is_partitioned | sym :: is_sorted_by_key | sym :: map | sym :: map_while | sym :: position | sym :: rposition | sym :: try_for_each) && let ExprKind :: Closure (closure) = arg . kind && let body = cx . tcx . hir_body (closure . body) && let [param] = body . params { Some (param . pat) } else { None } }
};
}
