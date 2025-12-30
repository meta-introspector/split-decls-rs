// Generated macro for parameters_for_impl (function)
macro_rules! Depcrate_constrained_generic_paramsparameters_for_impl {
() => {
// Module: crate::constrained_generic_params
// Provides: {"parameters_for_impl"}
// Dependencies: {}
# [doc = " Returns the set of parameters constrained by the impl header."] pub (crate) fn parameters_for_impl < 'tcx > (tcx : TyCtxt < 'tcx > , impl_self_ty : Ty < 'tcx > , impl_trait_ref : Option < ty :: TraitRef < 'tcx > > ,) -> FxHashSet < Parameter > { let vec = match impl_trait_ref { Some (tr) => parameters_for (tcx , tr , false) , None => parameters_for (tcx , impl_self_ty , false) , } ; vec . into_iter () . collect () }
};
}
