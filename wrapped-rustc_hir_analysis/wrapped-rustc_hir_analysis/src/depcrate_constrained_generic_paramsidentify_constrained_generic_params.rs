// Generated macro for identify_constrained_generic_params (function)
macro_rules! Depcrate_constrained_generic_paramsidentify_constrained_generic_params {
() => {
// Module: crate::constrained_generic_params
// Provides: {"identify_constrained_generic_params"}
// Dependencies: {}
pub (crate) fn identify_constrained_generic_params < 'tcx > (tcx : TyCtxt < 'tcx > , predicates : ty :: GenericPredicates < 'tcx > , impl_trait_ref : Option < ty :: TraitRef < 'tcx > > , input_parameters : & mut FxHashSet < Parameter > ,) { let mut predicates = predicates . predicates . to_vec () ; setup_constraining_predicates (tcx , & mut predicates , impl_trait_ref , input_parameters) ; }
};
}
