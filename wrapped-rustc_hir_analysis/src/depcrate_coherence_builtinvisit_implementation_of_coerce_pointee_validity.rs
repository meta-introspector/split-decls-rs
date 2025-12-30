// Generated macro for visit_implementation_of_coerce_pointee_validity (function)
macro_rules! Depcrate_coherence_builtinvisit_implementation_of_coerce_pointee_validity {
() => {
// Module: crate::coherence::builtin
// Provides: {"visit_implementation_of_coerce_pointee_validity"}
// Dependencies: {}
fn visit_implementation_of_coerce_pointee_validity (checker : & Checker < '_ > ,) -> Result < () , ErrorGuaranteed > { let tcx = checker . tcx ; let self_ty = tcx . impl_trait_ref (checker . impl_def_id) . unwrap () . instantiate_identity () . self_ty () ; let span = tcx . def_span (checker . impl_def_id) ; if ! tcx . is_builtin_derived (checker . impl_def_id . into ()) { return Err (tcx . dcx () . emit_err (errors :: CoercePointeeNoUserValidityAssertion { span })) ; } let ty :: Adt (def , _args) = self_ty . kind () else { return Err (tcx . dcx () . emit_err (errors :: CoercePointeeNotConcreteType { span })) ; } ; let did = def . did () ; let span = tcx . def_span (did) ; if ! def . is_struct () { return Err (tcx . dcx () . emit_err (errors :: CoercePointeeNotStruct { span , kind : def . descr () . into () })) ; } if ! def . repr () . transparent () { return Err (tcx . dcx () . emit_err (errors :: CoercePointeeNotTransparent { span })) ; } if def . all_fields () . next () . is_none () { return Err (tcx . dcx () . emit_err (errors :: CoercePointeeNoField { span })) ; } Ok (()) }
};
}
