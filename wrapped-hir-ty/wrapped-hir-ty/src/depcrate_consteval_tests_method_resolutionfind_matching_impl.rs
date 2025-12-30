// Generated macro for find_matching_impl (function)
macro_rules! Depcrate_consteval_tests_method_resolutionfind_matching_impl {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"find_matching_impl"}
// Dependencies: {}
pub (crate) fn find_matching_impl < 'db > (infcx : & InferCtxt < 'db > , env : & TraitEnvironment < 'db > , trait_ref : TraitRef < 'db > ,) -> Option < (ImplId , GenericArgs < 'db >) > { let trait_ref = infcx . at (& ObligationCause :: dummy () , env . env) . deeply_normalize (trait_ref) . ok () ? ; let obligation = Obligation :: new (infcx . interner , ObligationCause :: dummy () , env . env , trait_ref) ; let selection = infcx . select (& obligation) . ok () ? ? ; let mut ocx = ObligationCtxt :: new (infcx) ; let impl_source = selection . map (| obligation | ocx . register_obligation (obligation)) ; let errors = ocx . evaluate_obligations_error_on_ambiguity () ; if ! errors . is_empty () { return None ; } let impl_source = infcx . resolve_vars_if_possible (impl_source) ; if impl_source . has_non_region_infer () { return None ; } match impl_source { ImplSource :: UserDefined (impl_source) => Some ((impl_source . impl_def_id , impl_source . args)) , ImplSource :: Param (_) | ImplSource :: Builtin (..) => None , } }
};
}
