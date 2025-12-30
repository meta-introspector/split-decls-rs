// Generated macro for check_negative_auto_trait_impl (function)
macro_rules! Depcrate_check_always_applicablecheck_negative_auto_trait_impl {
() => {
// Module: crate::check::always_applicable
// Provides: {"check_negative_auto_trait_impl"}
// Dependencies: {}
pub (crate) fn check_negative_auto_trait_impl < 'tcx > (tcx : TyCtxt < 'tcx > , impl_def_id : LocalDefId , impl_trait_ref : ty :: TraitRef < 'tcx > , polarity : ty :: ImplPolarity ,) -> Result < () , ErrorGuaranteed > { let ty :: ImplPolarity :: Negative = polarity else { return Ok (()) ; } ; if ! tcx . trait_is_auto (impl_trait_ref . def_id) { return Ok (()) ; } if tcx . defaultness (impl_def_id) . is_default () { tcx . dcx () . span_delayed_bug (tcx . def_span (impl_def_id) , "default impl cannot be negative") ; } tcx . ensure_ok () . orphan_check_impl (impl_def_id) ? ; match impl_trait_ref . self_ty () . kind () { ty :: Adt (adt_def , adt_to_impl_args) => { ensure_impl_params_and_item_params_correspond (tcx , impl_def_id , adt_def . did () , adt_to_impl_args ,) ? ; ensure_impl_predicates_are_implied_by_item_defn (tcx , impl_def_id , adt_def . did () , adt_to_impl_args ,) } _ => { if tcx . features () . auto_traits () { Ok (()) } else { Err (tcx . dcx () . span_delayed_bug (tcx . def_span (impl_def_id) , "incoherent impl of negative auto trait" ,)) } } } }
};
}
