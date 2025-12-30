// Generated macro for make_projection (function)
macro_rules! Depcrate_tymake_projection {
() => {
// Module: crate::ty
// Provides: {"make_projection"}
// Dependencies: {}
# [doc = " Makes the projection type for the named associated type in the given impl or trait impl."] # [doc = ""] # [doc = " This function is for associated types which are \"known\" to exist, and as such, will only return"] # [doc = " `None` when debug assertions are disabled in order to prevent ICE's. With debug assertions"] # [doc = " enabled this will check that the named associated type exists, the correct number of"] # [doc = " arguments are given, and that the correct kinds of arguments are given (lifetime,"] # [doc = " constant or type). This will not check if type normalization would succeed."] pub fn make_projection < 'tcx > (tcx : TyCtxt < 'tcx > , container_id : DefId , assoc_ty : Symbol , args : impl IntoIterator < Item = impl Into < GenericArg < 'tcx > > > ,) -> Option < AliasTy < 'tcx > > { fn helper < 'tcx > (tcx : TyCtxt < 'tcx > , container_id : DefId , assoc_ty : Symbol , args : GenericArgsRef < 'tcx > ,) -> Option < AliasTy < 'tcx > > { let Some (assoc_item) = tcx . associated_items (container_id) . find_by_ident_and_kind (tcx , Ident :: with_dummy_span (assoc_ty) , AssocTag :: Type , container_id ,) else { debug_assert ! (false , "type `{assoc_ty}` not found in `{container_id:?}`") ; return None ; } ; # [cfg (debug_assertions)] assert_generic_args_match (tcx , assoc_item . def_id , args) ; Some (AliasTy :: new_from_args (tcx , assoc_item . def_id , args)) } helper (tcx , container_id , assoc_ty , tcx . mk_args_from_iter (args . into_iter () . map (Into :: into)) ,) }
};
}
