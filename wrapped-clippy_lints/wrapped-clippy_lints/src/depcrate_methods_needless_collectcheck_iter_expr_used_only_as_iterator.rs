// Generated macro for check_iter_expr_used_only_as_iterator (function)
macro_rules! Depcrate_methods_needless_collectcheck_iter_expr_used_only_as_iterator {
() => {
// Module: crate::methods::needless_collect
// Provides: {"check_iter_expr_used_only_as_iterator"}
// Dependencies: {}
fn check_iter_expr_used_only_as_iterator < 'tcx > (cx : & LateContext < 'tcx > , hir_id_of_expr : HirId , block : & 'tcx Block < 'tcx > ,) -> bool { let mut visitor = IteratorMethodCheckVisitor { cx , hir_id_of_expr , hir_id_of_let_binding : None , } ; visitor . visit_block (block) . is_continue () }
};
}
