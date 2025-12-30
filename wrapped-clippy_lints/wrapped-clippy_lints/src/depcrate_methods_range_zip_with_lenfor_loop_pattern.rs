// Generated macro for for_loop_pattern (function)
macro_rules! Depcrate_methods_range_zip_with_lenfor_loop_pattern {
() => {
// Module: crate::methods::range_zip_with_len
// Provides: {"for_loop_pattern"}
// Dependencies: {}
# [doc = " If `expr` is the argument of a `for` loop, return the loop pattern."] fn for_loop_pattern < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> Option < & 'tcx Pat < 'tcx > > { cx . tcx . hir_parent_iter (expr . hir_id) . find_map (| (_ , node) | { if let Node :: Expr (ancestor_expr) = node && let Some (for_loop) = higher :: ForLoop :: hir (ancestor_expr) && for_loop . arg . hir_id == expr . hir_id { Some (for_loop . pat) } else { None } }) }
};
}
