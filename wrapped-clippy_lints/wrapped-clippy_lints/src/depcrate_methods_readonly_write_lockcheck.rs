// Generated macro for check (function)
macro_rules! Depcrate_methods_readonly_write_lockcheck {
() => {
// Module: crate::methods::readonly_write_lock
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > , receiver : & Expr < '_ >) { if cx . typeck_results () . expr_ty (receiver) . peel_refs () . is_diag_item (cx , sym :: RwLock) && let Node :: Expr (unwrap_call_expr) = cx . tcx . parent_hir_node (expr . hir_id) && is_unwrap_call (cx , unwrap_call_expr) && let parent = cx . tcx . parent_hir_node (unwrap_call_expr . hir_id) && let Node :: LetStmt (local) = parent && let PatKind :: Binding (.. , ident , _) = local . pat . kind && ! ident . as_str () . starts_with ('_') && let Some (mir) = enclosing_mir (cx . tcx , expr . hir_id) && let Some ((local , _)) = mir . local_decls . iter_enumerated () . find (| (_ , decl) | local . span . contains (decl . source_info . span)) && let Some (usages) = visit_local_usage (& [local] , mir , Location { block : START_BLOCK , statement_index : 0 , } ,) && let [usage] = usages . as_slice () { let writer_never_mutated = usage . local_consume_or_mutate_locs . is_empty () ; if writer_never_mutated { span_lint_and_sugg (cx , READONLY_WRITE_LOCK , expr . span , "this write lock is used only for reading" , "consider using a read lock instead" , format ! ("{}.read()" , snippet (cx , receiver . span , "<receiver>")) , Applicability :: MaybeIncorrect ,) ; } } }
};
}
