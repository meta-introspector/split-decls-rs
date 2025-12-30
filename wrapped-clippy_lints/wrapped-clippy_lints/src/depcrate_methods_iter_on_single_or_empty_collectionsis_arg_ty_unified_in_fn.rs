// Generated macro for is_arg_ty_unified_in_fn (function)
macro_rules! Depcrate_methods_iter_on_single_or_empty_collectionsis_arg_ty_unified_in_fn {
() => {
// Module: crate::methods::iter_on_single_or_empty_collections
// Provides: {"is_arg_ty_unified_in_fn"}
// Dependencies: {}
fn is_arg_ty_unified_in_fn < 'tcx > (cx : & LateContext < 'tcx > , fn_sig : ExprFnSig < 'tcx > , arg_id : HirId , args : impl IntoIterator < Item = & 'tcx Expr < 'tcx > > , is_method : bool ,) -> bool { let arg_id_in_args = args . into_iter () . position (| e | e . hir_id == arg_id) . unwrap () ; let Some (arg_ty_in_args) = fn_sig . input (arg_id_in_args) . map (Binder :: skip_binder) else { return false ; } ; fn_sig . predicates_id () . map (| def_id | cx . tcx . predicates_of (def_id)) . is_some_and (| generics | { generics . predicates . iter () . any (| (clause , _) | { clause . as_projection_clause () . and_then (| p | p . map_bound (| p | p . term . as_type ()) . transpose ()) . is_some_and (| ty | ty . skip_binder () == arg_ty_in_args) }) }) || (! is_method && fn_sig . input (arg_id_in_args) . is_some_and (| binder | { binder . skip_binder () . walk () . any (| arg | arg . as_type () == Some (arg_ty_in_args)) })) }
};
}
