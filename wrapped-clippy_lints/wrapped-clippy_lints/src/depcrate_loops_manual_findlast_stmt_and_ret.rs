// Generated macro for last_stmt_and_ret (function)
macro_rules! Depcrate_loops_manual_findlast_stmt_and_ret {
() => {
// Module: crate::loops::manual_find
// Provides: {"last_stmt_and_ret"}
// Dependencies: {}
fn last_stmt_and_ret < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ > ,) -> Option < (& 'tcx Stmt < 'tcx > , & 'tcx Expr < 'tcx >) > { fn extract < 'tcx > (block : & Block < 'tcx >) -> Option < (& 'tcx Stmt < 'tcx > , & 'tcx Expr < 'tcx >) > { if let [.. , last_stmt] = block . stmts { if let Some (ret) = block . expr { return Some ((last_stmt , ret)) ; } if let [.. , snd_last , _] = block . stmts && let StmtKind :: Semi (last_expr) = last_stmt . kind && let ExprKind :: Ret (Some (ret)) = last_expr . kind { return Some ((snd_last , ret)) ; } } None } let mut parent_iter = cx . tcx . hir_parent_iter (expr . hir_id) ; if let Some ((node_hir , Node :: Stmt (..))) = parent_iter . next () && let Some ((_ , Node :: Block (block))) = parent_iter . next () && let Some ((last_stmt , last_ret)) = extract (block) && last_stmt . hir_id == node_hir && last_ret . res (cx) . ctor_parent (cx) . is_lang_item (cx , LangItem :: OptionNone) && let Some ((_ , Node :: Expr (_block))) = parent_iter . next () && let Some ((_ , func)) = parent_iter . next () && func . fn_kind () . is_some () { Some ((block . stmts . last () . unwrap () , last_ret)) } else { None } }
};
}
