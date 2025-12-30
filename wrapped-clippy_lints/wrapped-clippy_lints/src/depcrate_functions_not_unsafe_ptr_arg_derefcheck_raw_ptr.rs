// Generated macro for check_raw_ptr (function)
macro_rules! Depcrate_functions_not_unsafe_ptr_arg_derefcheck_raw_ptr {
() => {
// Module: crate::functions::not_unsafe_ptr_arg_deref
// Provides: {"check_raw_ptr"}
// Dependencies: {}
fn check_raw_ptr < 'tcx > (cx : & LateContext < 'tcx > , safety : hir :: Safety , decl : & 'tcx hir :: FnDecl < 'tcx > , body : & 'tcx hir :: Body < 'tcx > , def_id : LocalDefId ,) { if safety . is_safe () && cx . effective_visibilities . is_exported (def_id) { let raw_ptrs = iter_input_pats (decl , body) . filter_map (| arg | raw_ptr_arg (cx , arg)) . collect :: < HirIdSet > () ; if ! raw_ptrs . is_empty () { let typeck = cx . tcx . typeck_body (body . id ()) ; let _ : Option < ! > = for_each_expr (cx , body . value , | e | { match e . kind { hir :: ExprKind :: Call (f , args) if is_unsafe_fn (cx , typeck . expr_ty (f)) => { for arg in args { check_arg (cx , & raw_ptrs , arg) ; } } , hir :: ExprKind :: MethodCall (_ , recv , args , _) => { let def_id = typeck . type_dependent_def_id (e . hir_id) . unwrap () ; if cx . tcx . fn_sig (def_id) . skip_binder () . skip_binder () . safety . is_unsafe () { check_arg (cx , & raw_ptrs , recv) ; for arg in args { check_arg (cx , & raw_ptrs , arg) ; } } } , hir :: ExprKind :: Unary (hir :: UnOp :: Deref , ptr) => check_arg (cx , & raw_ptrs , ptr) , _ => () , } ControlFlow :: Continue (()) }) ; } } }
};
}
