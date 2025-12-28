macro_rules! deps {
    () => {
        Parameter!();
    };
}

macro_rules! identify_constrained_generic_params {
    () => {
        deps!();
        pub (crate) fn identify_constrained_generic_params < 'tcx > (tcx : TyCtxt < 'tcx > , predicates : ty :: GenericPredicates < 'tcx > , impl_trait_ref : Option < ty :: TraitRef < 'tcx > > , input_parameters : & mut FxHashSet < Parameter > ,) { let mut predicates = predicates . predicates . to_vec () ; setup_constraining_predicates (tcx , & mut predicates , impl_trait_ref , input_parameters) ; }
    };
}

identify_constrained_generic_params!()