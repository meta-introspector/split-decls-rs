// Generated macro for impl_6047 (impl)
macro_rules! Depcrate_methods_needless_collectimpl_6047 {
() => {
// Module: crate::methods::needless_collect
// Provides: {"impl_6047"}
// Dependencies: {}
impl < 'tcx > IterFunctionVisitor < '_ , 'tcx > { fn visit_block_expr (& mut self , expr : & 'tcx Expr < 'tcx > , hir_id : Option < HirId >) { self . current_statement_hir_id = hir_id ; self . current_mutably_captured_ids = get_captured_ids (self . cx , self . cx . typeck_results () . expr_ty (expr)) ; self . visit_expr (expr) ; } }
};
}
