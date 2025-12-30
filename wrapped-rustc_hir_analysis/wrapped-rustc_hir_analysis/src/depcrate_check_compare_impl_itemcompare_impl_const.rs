// Generated macro for compare_impl_const (function)
macro_rules! Depcrate_check_compare_impl_itemcompare_impl_const {
() => {
// Module: crate::check::compare_impl_item
// Provides: {"compare_impl_const"}
// Dependencies: {}
fn compare_impl_const < 'tcx > (tcx : TyCtxt < 'tcx > , impl_const_item : ty :: AssocItem , trait_const_item : ty :: AssocItem , impl_trait_ref : ty :: TraitRef < 'tcx > ,) -> Result < () , ErrorGuaranteed > { compare_number_of_generics (tcx , impl_const_item , trait_const_item , false) ? ; compare_generic_param_kinds (tcx , impl_const_item , trait_const_item , false) ? ; check_region_bounds_on_impl_item (tcx , impl_const_item , trait_const_item , false) ? ; compare_const_predicate_entailment (tcx , impl_const_item , trait_const_item , impl_trait_ref) }
};
}
