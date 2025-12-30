// Generated macro for impl_2097 (impl)
macro_rules! Depcrate_exitimpl_2097 {
() => {
// Module: crate::exit
// Provides: {"impl_2097"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Exit { fn check_expr (& mut self , cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ >) { if let ExprKind :: Call (path_expr , [_]) = e . kind && let ExprKind :: Path (ref path) = path_expr . kind && let Some (def_id) = cx . qpath_res (path , path_expr . hir_id) . opt_def_id () && cx . tcx . is_diagnostic_item (sym :: process_exit , def_id) && let parent = cx . tcx . hir_get_parent_item (e . hir_id) && let OwnerNode :: Item (Item { kind : ItemKind :: Fn { ident , .. } , .. }) = cx . tcx . hir_owner_node (parent) && ident . name != sym :: main { span_lint (cx , EXIT , e . span , "usage of `process::exit`") ; } } }
};
}
