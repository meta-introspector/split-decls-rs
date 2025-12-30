// Generated macro for is_ty_const_destruct (function)
macro_rules! Depcrate_qualify_min_const_fnis_ty_const_destruct {
() => {
// Module: crate::qualify_min_const_fn
// Provides: {"is_ty_const_destruct"}
// Dependencies: {}
fn is_ty_const_destruct < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , body : & Body < 'tcx >) -> bool { # [expect (unused)] fn is_ty_const_destruct_unused < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , body : & Body < 'tcx >) -> bool { if ! ty . needs_drop (tcx , body . typing_env (tcx)) { return false ; } let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (body . typing_env (tcx)) ; let obligation = Obligation :: new (tcx , ObligationCause :: dummy_with_span (body . span) , param_env , TraitRef :: new (tcx , tcx . require_lang_item (LangItem :: Destruct , body . span) , [ty]) ,) ; let mut selcx = SelectionContext :: new (& infcx) ; let Some (impl_src) = selcx . select (& obligation) . ok () . flatten () else { return false ; } ; if ! matches ! (impl_src , ImplSource :: Builtin (BuiltinImplSource :: Misc , _) | ImplSource :: Param (_)) { return false ; } let ocx = ObligationCtxt :: new (& infcx) ; ocx . register_obligations (impl_src . nested_obligations ()) ; ocx . evaluate_obligations_error_on_ambiguity () . is_empty () } ! ty . needs_drop (tcx , ConstCx :: new (tcx , body) . typing_env) }
};
}
