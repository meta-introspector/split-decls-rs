macro_rules! compare_impl_method {
    () => {
        # [doc = " Checks that a method from an impl conforms to the signature of"] # [doc = " the same method as declared in the trait."] # [doc = ""] # [doc = " # Parameters"] # [doc = ""] # [doc = " - `impl_m`: type of the method we are checking"] # [doc = " - `trait_m`: the method in the trait"] # [doc = " - `impl_trait_ref`: the TraitRef corresponding to the trait implementation"] # [instrument (level = "debug" , skip (tcx))] fn compare_impl_method < 'tcx > (tcx : TyCtxt < 'tcx > , impl_m : ty :: AssocItem , trait_m : ty :: AssocItem , impl_trait_ref : ty :: TraitRef < 'tcx > ,) -> Result < () , ErrorGuaranteed > { check_method_is_structurally_compatible (tcx , impl_m , trait_m , impl_trait_ref , false) ? ; compare_method_predicate_entailment (tcx , impl_m , trait_m , impl_trait_ref) ? ; Ok (()) }
    };
}

compare_impl_method!();