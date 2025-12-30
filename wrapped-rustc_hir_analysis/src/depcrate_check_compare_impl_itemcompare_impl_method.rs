// Generated macro for compare_impl_method (function)
macro_rules! Depcrate_check_compare_impl_itemcompare_impl_method {
() => {
// Module: crate::check::compare_impl_item
// Provides: {"compare_impl_method"}
// Dependencies: {}
# [doc = " Checks that a method from an impl conforms to the signature of"] # [doc = " the same method as declared in the trait."] # [doc = ""] # [doc = " # Parameters"] # [doc = ""] # [doc = " - `impl_m`: type of the method we are checking"] # [doc = " - `trait_m`: the method in the trait"] # [doc = " - `impl_trait_ref`: the TraitRef corresponding to the trait implementation"] # [instrument (level = "debug" , skip (tcx))] fn compare_impl_method < 'tcx > (tcx : TyCtxt < 'tcx > , impl_m : ty :: AssocItem , trait_m : ty :: AssocItem , impl_trait_ref : ty :: TraitRef < 'tcx > ,) -> Result < () , ErrorGuaranteed > { check_method_is_structurally_compatible (tcx , impl_m , trait_m , impl_trait_ref , false) ? ; compare_method_predicate_entailment (tcx , impl_m , trait_m , impl_trait_ref) ? ; Ok (()) }
};
}
