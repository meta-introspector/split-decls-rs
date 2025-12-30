// Generated macro for is_min_const_fn (function)
macro_rules! Depcrate_qualify_min_const_fnis_min_const_fn {
() => {
// Module: crate::qualify_min_const_fn
// Provides: {"is_min_const_fn"}
// Dependencies: {}
pub fn is_min_const_fn < 'tcx > (cx : & LateContext < 'tcx > , body : & Body < 'tcx > , msrv : Msrv) -> McfResult { let def_id = body . source . def_id () ; for local in & body . local_decls { check_ty (cx , local . ty , local . source_info . span , msrv) ? ; } if ! msrv . meets (cx , msrvs :: CONST_FN_TRAIT_BOUND) && let Some (sized_did) = cx . tcx . lang_items () . sized_trait () && let Some (meta_sized_did) = cx . tcx . lang_items () . meta_sized_trait () && cx . tcx . param_env (def_id) . caller_bounds () . iter () . any (| bound | { bound . as_trait_clause () . is_some_and (| clause | { let did = clause . def_id () ; did != sized_did && did != meta_sized_did }) }) { return Err ((body . span , "non-`Sized` trait clause before `const_fn_trait_bound` is stabilized" . into () ,)) ; } check_ty (cx , cx . tcx . fn_sig (def_id) . instantiate_identity () . output () . skip_binder () , body . local_decls . iter () . next () . unwrap () . source_info . span , msrv ,) ? ; for bb in & * body . basic_blocks { if ! bb . is_cleanup { check_terminator (cx , body , bb . terminator () , msrv) ? ; for stmt in & bb . statements { check_statement (cx , body , def_id , stmt , msrv) ? ; } } } Ok (()) }
};
}
