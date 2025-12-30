// Generated macro for handle_path (function)
macro_rules! Depcrate_methods_map_clonehandle_path {
() => {
// Module: crate::methods::map_clone
// Provides: {"handle_path"}
// Dependencies: {}
fn handle_path (cx : & LateContext < '_ > , arg : & hir :: Expr < '_ > , qpath : & hir :: QPath < '_ > , e : & hir :: Expr < '_ > , recv : & hir :: Expr < '_ > ,) { if let Some (path_def_id) = cx . qpath_res (qpath , arg . hir_id) . opt_def_id () && cx . tcx . lang_items () . get (LangItem :: CloneFn) == Some (path_def_id) && let ty :: Adt (_ , args) = cx . typeck_results () . expr_ty (recv) . kind () && let args = args . as_slice () && let Some (ty) = args . iter () . find_map (| generic_arg | generic_arg . as_type ()) && let ty :: Ref (_ , ty , Mutability :: Not) = ty . kind () && let ty :: FnDef (_ , lst) = cx . typeck_results () . expr_ty (arg) . kind () && lst . iter () . all (| l | l . as_type () == Some (* ty)) && ! should_call_clone_as_function (cx , * ty) { lint_path (cx , e . span , recv . span , is_copy (cx , ty . peel_refs ())) ; } }
};
}
