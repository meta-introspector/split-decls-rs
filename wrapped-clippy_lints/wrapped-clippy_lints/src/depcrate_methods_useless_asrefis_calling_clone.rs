// Generated macro for is_calling_clone (function)
macro_rules! Depcrate_methods_useless_asrefis_calling_clone {
() => {
// Module: crate::methods::useless_asref
// Provides: {"is_calling_clone"}
// Dependencies: {}
fn is_calling_clone (cx : & LateContext < '_ > , arg : & hir :: Expr < '_ >) -> bool { match arg . kind { hir :: ExprKind :: Closure (& hir :: Closure { body , .. }) if let closure_body = cx . tcx . hir_body (body) && let [param] = closure_body . params && let hir :: PatKind :: Binding (_ , local_id , ..) = strip_pat_refs (param . pat) . kind => { let closure_expr = peel_blocks (closure_body . value) ; match closure_expr . kind { hir :: ExprKind :: MethodCall (method , obj , [] , _) => { if method . ident . name == sym :: clone && let Some (fn_id) = cx . typeck_results () . type_dependent_def_id (closure_expr . hir_id) && let Some (trait_id) = cx . tcx . trait_of_assoc (fn_id) && cx . tcx . lang_items () . clone_trait () . is_some_and (| id | id == trait_id) && ! cx . typeck_results () . expr_adjustments (obj) . iter () . any (| a | matches ! (a . kind , Adjust :: Deref (Some (..)))) && obj . res_local_id () == Some (local_id) { true } else { false } } , hir :: ExprKind :: Call (call , [recv]) => { if let hir :: ExprKind :: Path (qpath) = call . kind && recv . res_local_id () == Some (local_id) { check_qpath (cx , qpath , call . hir_id) } else { false } } , _ => false , } } , hir :: ExprKind :: Path (qpath) => check_qpath (cx , qpath , arg . hir_id) , _ => false , } }
};
}
