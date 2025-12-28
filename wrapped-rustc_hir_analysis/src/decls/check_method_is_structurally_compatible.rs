macro_rules! check_method_is_structurally_compatible {
    () => {
        # [doc = " Checks a bunch of different properties of the impl/trait methods for"] # [doc = " compatibility, such as asyncness, number of argument, self receiver kind,"] # [doc = " and number of early- and late-bound generics."] fn check_method_is_structurally_compatible < 'tcx > (tcx : TyCtxt < 'tcx > , impl_m : ty :: AssocItem , trait_m : ty :: AssocItem , impl_trait_ref : ty :: TraitRef < 'tcx > , delay : bool ,) -> Result < () , ErrorGuaranteed > { compare_self_type (tcx , impl_m , trait_m , impl_trait_ref , delay) ? ; compare_number_of_generics (tcx , impl_m , trait_m , delay) ? ; compare_generic_param_kinds (tcx , impl_m , trait_m , delay) ? ; compare_number_of_method_arguments (tcx , impl_m , trait_m , delay) ? ; compare_synthetic_generics (tcx , impl_m , trait_m , delay) ? ; check_region_bounds_on_impl_item (tcx , impl_m , trait_m , delay) ? ; Ok (()) }
    };
}

check_method_is_structurally_compatible!();