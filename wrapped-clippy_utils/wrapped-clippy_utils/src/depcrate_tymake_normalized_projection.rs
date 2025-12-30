// Generated macro for make_normalized_projection (function)
macro_rules! Depcrate_tymake_normalized_projection {
() => {
// Module: crate::ty
// Provides: {"make_normalized_projection"}
// Dependencies: {}
# [doc = " Normalizes the named associated type in the given impl or trait impl."] # [doc = ""] # [doc = " This function is for associated types which are \"known\" to be valid with the given"] # [doc = " arguments, and as such, will only return `None` when debug assertions are disabled in order"] # [doc = " to prevent ICE's. With debug assertions enabled this will check that type normalization"] # [doc = " succeeds as well as everything checked by `make_projection`."] pub fn make_normalized_projection < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , container_id : DefId , assoc_ty : Symbol , args : impl IntoIterator < Item = impl Into < GenericArg < 'tcx > > > ,) -> Option < Ty < 'tcx > > { fn helper < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : AliasTy < 'tcx >) -> Option < Ty < 'tcx > > { # [cfg (debug_assertions)] if let Some ((i , arg)) = ty . args . iter () . enumerate () . find (| (_ , arg) | arg . has_escaping_bound_vars ()) { debug_assert ! (false , "args contain late-bound region at index `{i}` which can't be normalized.\n\
                    use `TyCtxt::instantiate_bound_regions_with_erased`\n\
                    note: arg is `{arg:#?}`" ,) ; return None ; } match tcx . try_normalize_erasing_regions (typing_env , Ty :: new_projection_from_args (tcx , ty . def_id , ty . args)) { Ok (ty) => Some (ty) , Err (e) => { debug_assert ! (false , "failed to normalize type `{ty}`: {e:#?}") ; None } , } } helper (tcx , typing_env , make_projection (tcx , container_id , assoc_ty , args) ?) }
};
}
