// Generated macro for impl_9773 (impl)
macro_rules! Depcrate_significant_drop_tighteningimpl_9773 {
() => {
// Module: crate::significant_drop_tightening
// Provides: {"impl_9773"}
// Dependencies: {}
impl < 'ap , 'lc , 'others , 'stmt , 'tcx > StmtsChecker < 'ap , 'lc , 'others , 'stmt , 'tcx > { fn new (ap : & 'ap mut AuxParams < 'others , 'stmt , 'tcx > , cx : & 'lc LateContext < 'tcx > , type_cache : & 'others mut FxHashMap < Ty < 'tcx > , bool > ,) -> Self { Self { ap , cx , type_cache } } fn manage_has_expensive_expr_after_last_attr (& mut self) { let has_expensive_stmt = match self . ap . curr_stmt . kind { hir :: StmtKind :: Expr (expr) if is_inexpensive_expr (expr) => false , hir :: StmtKind :: Let (local) if let Some (expr) = local . init && let hir :: ExprKind :: Path (_) = expr . kind => { false } , _ => true , } ; if has_expensive_stmt { for apa in self . ap . apas . values_mut () { let last_stmt_is_not_dummy = apa . last_stmt_span != DUMMY_SP ; let last_stmt_is_not_curr = self . ap . curr_stmt . span != apa . last_stmt_span ; let block_equals_curr = self . ap . curr_block_hir_id == apa . first_block_hir_id ; let block_is_ancestor = self . cx . tcx . hir_parent_iter (self . ap . curr_block_hir_id) . any (| (id , _) | id == apa . first_block_hir_id) ; if last_stmt_is_not_dummy && last_stmt_is_not_curr && (block_equals_curr || block_is_ancestor) { apa . has_expensive_expr_after_last_attr = true ; } } } } }
};
}
