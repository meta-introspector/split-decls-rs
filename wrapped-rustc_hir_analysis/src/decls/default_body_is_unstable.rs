macro_rules! deps {
    () => {
        MissingTraitItemUnstable!();
    };
}

macro_rules! default_body_is_unstable {
    () => {
        deps!();
        fn default_body_is_unstable (tcx : TyCtxt < '_ > , impl_span : Span , item_did : DefId , feature : Symbol , reason : Option < Symbol > , issue : Option < NonZero < u32 > > ,) { let missing_item_name = tcx . item_ident (item_did) ; let (mut some_note , mut none_note , mut reason_str) = (false , false , String :: new ()) ; match reason { Some (r) => { some_note = true ; reason_str = r . to_string () ; } None => none_note = true , } ; let mut err = tcx . dcx () . create_err (errors :: MissingTraitItemUnstable { span : impl_span , some_note , none_note , missing_item_name , feature , reason : reason_str , }) ; let inject_span = item_did . is_local () . then (| | tcx . crate_level_attribute_injection_span ()) ; rustc_session :: parse :: add_feature_diagnostics_for_issue (& mut err , & tcx . sess , feature , rustc_feature :: GateIssue :: Library (issue) , false , inject_span ,) ; err . emit () ; }
    };
}

default_body_is_unstable!();