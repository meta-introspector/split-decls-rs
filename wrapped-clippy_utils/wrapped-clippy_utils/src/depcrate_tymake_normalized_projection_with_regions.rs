// Generated macro for make_normalized_projection_with_regions (function)
macro_rules! Depcrate_tymake_normalized_projection_with_regions {
() => {
// Module: crate::ty
// Provides: {"make_normalized_projection_with_regions"}
// Dependencies: {}
pub fn make_normalized_projection_with_regions < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , container_id : DefId , assoc_ty : Symbol , args : impl IntoIterator < Item = impl Into < GenericArg < 'tcx > > > ,) -> Option < Ty < 'tcx > > { fn helper < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : AliasTy < 'tcx >) -> Option < Ty < 'tcx > > { # [cfg (debug_assertions)] if let Some ((i , arg)) = ty . args . iter () . enumerate () . find (| (_ , arg) | arg . has_escaping_bound_vars ()) { debug_assert ! (false , "args contain late-bound region at index `{i}` which can't be normalized.\n\
                    use `TyCtxt::instantiate_bound_regions_with_erased`\n\
                    note: arg is `{arg:#?}`" ,) ; return None ; } let cause = ObligationCause :: dummy () ; let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (typing_env) ; match infcx . at (& cause , param_env) . query_normalize (Ty :: new_projection_from_args (tcx , ty . def_id , ty . args)) { Ok (ty) => Some (ty . value) , Err (e) => { debug_assert ! (false , "failed to normalize type `{ty}`: {e:#?}") ; None } , } } helper (tcx , typing_env , make_projection (tcx , container_id , assoc_ty , args) ?) }
};
}
