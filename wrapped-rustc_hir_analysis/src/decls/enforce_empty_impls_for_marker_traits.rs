macro_rules! enforce_empty_impls_for_marker_traits {
    () => {
        # [doc = " We allow impls of marker traits to overlap, so they can't override impls"] # [doc = " as that could make it ambiguous which associated item to use."] fn enforce_empty_impls_for_marker_traits (tcx : TyCtxt < '_ > , impl_def_id : LocalDefId , trait_def_id : DefId , trait_def : & ty :: TraitDef ,) -> Result < () , ErrorGuaranteed > { if ! trait_def . is_marker { return Ok (()) ; } if tcx . associated_item_def_ids (trait_def_id) . is_empty () { return Ok (()) ; } Err (struct_span_code_err ! (tcx . dcx () , tcx . def_span (impl_def_id) , E0715 , "impls for marker traits cannot contain items") . emit ()) }
    };
}

enforce_empty_impls_for_marker_traits!()