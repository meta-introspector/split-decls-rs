// Generated macro for needs_turbofish (function)
macro_rules! Depcrate_methods_unnecessary_foldneeds_turbofish {
() => {
// Module: crate::methods::unnecessary_fold
// Provides: {"needs_turbofish"}
// Dependencies: {}
# [doc = " Do we need to suggest turbofish when suggesting a replacement method?"] # [doc = " Changing `fold` to `sum` needs it sometimes when the return type can't be"] # [doc = " inferred. This checks for some common cases where it can be safely omitted"] fn needs_turbofish (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ >) -> bool { let parent = cx . tcx . parent_hir_node (expr . hir_id) ; if let hir :: Node :: LetStmt (local) = parent && local . ty . is_some () { return false ; } if let hir :: Node :: Expr (parent_expr) = parent && let hir :: ExprKind :: Call (recv , args) = parent_expr . kind && let hir :: ExprKind :: Path (ref qpath) = recv . kind && let Some (fn_def_id) = cx . qpath_res (qpath , recv . hir_id) . opt_def_id () && let fn_sig = cx . tcx . fn_sig (fn_def_id) . skip_binder () . skip_binder () && let Some (arg_pos) = args . iter () . position (| arg | arg . hir_id == expr . hir_id) && let Some (ty) = fn_sig . inputs () . get (arg_pos) && ! matches ! (ty . kind () , ty :: Param (_)) { return false ; } true }
};
}
