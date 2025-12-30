// Generated macro for compare_impl_ty (function)
macro_rules! Depcrate_check_compare_impl_itemcompare_impl_ty {
() => {
// Module: crate::check::compare_impl_item
// Provides: {"compare_impl_ty"}
// Dependencies: {}
# [instrument (level = "debug" , skip (tcx))] fn compare_impl_ty < 'tcx > (tcx : TyCtxt < 'tcx > , impl_ty : ty :: AssocItem , trait_ty : ty :: AssocItem , impl_trait_ref : ty :: TraitRef < 'tcx > ,) -> Result < () , ErrorGuaranteed > { compare_number_of_generics (tcx , impl_ty , trait_ty , false) ? ; compare_generic_param_kinds (tcx , impl_ty , trait_ty , false) ? ; check_region_bounds_on_impl_item (tcx , impl_ty , trait_ty , false) ? ; compare_type_predicate_entailment (tcx , impl_ty , trait_ty , impl_trait_ref) ? ; check_type_bounds (tcx , trait_ty , impl_ty , impl_trait_ref) }
};
}
