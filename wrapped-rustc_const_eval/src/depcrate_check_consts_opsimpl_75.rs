// Generated macro for impl_75 (impl)
macro_rules! Depcrate_check_consts_opsimpl_75 {
() => {
// Module: crate::check_consts::ops
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'tcx > NonConstOp < 'tcx > for FnCallNonConst < 'tcx > { # [allow (rustc :: diagnostic_outside_of_impl)] # [allow (rustc :: untranslatable_diagnostic)] fn build_error (& self , ccx : & ConstCx < '_ , 'tcx > , _ : Span) -> Diag < 'tcx > { let tcx = ccx . tcx ; let caller = ccx . def_id () ; let mut err = build_error_for_const_call (ccx , self . callee , self . args , self . span , self . call_source , "non" , | err , self_ty , trait_id | { let trait_ref = TraitRef :: from_assoc (tcx , trait_id , self . args) ; match self_ty . kind () { Param (param_ty) => { debug ! (? param_ty) ; if let Some (generics) = tcx . hir_node_by_def_id (caller) . generics () { let constraint = with_no_trimmed_paths ! (format ! ("[const] {}" , trait_ref . print_trait_sugared () ,)) ; suggest_constraining_type_param (tcx , generics , err , param_ty . name . as_str () , & constraint , Some (trait_ref . def_id) , None ,) ; } } ty :: Adt (..) => { let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (ccx . typing_env) ; let obligation = Obligation :: new (tcx , ObligationCause :: dummy () , param_env , trait_ref) ; let mut selcx = SelectionContext :: new (& infcx) ; let implsrc = selcx . select (& obligation) ; if let Ok (Some (ImplSource :: UserDefined (data))) = implsrc { if ! tcx . is_const_trait_impl (data . impl_def_id) { let span = tcx . def_span (data . impl_def_id) ; err . subdiagnostic (errors :: NonConstImplNote { span }) ; } } } _ => { } } } ,) ; if let ConstContext :: Static (_) = ccx . const_kind () { err . note (fluent_generated :: const_eval_lazy_lock) ; } err } }
};
}
