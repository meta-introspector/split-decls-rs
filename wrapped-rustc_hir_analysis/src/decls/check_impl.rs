macro_rules! check_impl {
    () => {
        fn check_impl < 'tcx > (tcx : TyCtxt < 'tcx > , impl_def_id : LocalDefId , trait_ref : ty :: TraitRef < 'tcx > , trait_def : & 'tcx ty :: TraitDef , polarity : ty :: ImplPolarity ,) -> Result < () , ErrorGuaranteed > { debug ! ("(checking implementation) adding impl for trait '{:?}', item '{}'" , trait_ref , tcx . def_path_str (impl_def_id)) ; if trait_ref . references_error () { return Ok (()) ; } enforce_trait_manually_implementable (tcx , impl_def_id , trait_ref . def_id , trait_def) . and (enforce_empty_impls_for_marker_traits (tcx , impl_def_id , trait_ref . def_id , trait_def)) . and (always_applicable :: check_negative_auto_trait_impl (tcx , impl_def_id , trait_ref , polarity ,)) }
    };
}

check_impl!();