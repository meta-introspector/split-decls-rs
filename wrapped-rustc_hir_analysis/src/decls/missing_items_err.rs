macro_rules! deps {
    () => {
        MissingTraitItemLabel!();
        MissingTraitItemSuggestionNone!();
        MissingTraitItem!();
        MissingTraitItemSuggestion!();
    };
}

macro_rules! missing_items_err {
    () => {
        deps!();
        fn missing_items_err (tcx : TyCtxt < '_ > , impl_def_id : LocalDefId , missing_items : & [ty :: AssocItem] , full_impl_span : Span ,) { let missing_items = missing_items . iter () . filter (| trait_item | ! trait_item . is_impl_trait_in_trait ()) ; let missing_items_msg = missing_items . clone () . map (| trait_item | trait_item . name () . to_string ()) . collect :: < Vec < _ > > () . join ("`, `") ; let sugg_sp = if let Ok (snippet) = tcx . sess . source_map () . span_to_snippet (full_impl_span) && snippet . ends_with ("}") { let hi = full_impl_span . hi () - BytePos (1) ; full_impl_span . with_lo (hi) . with_hi (hi) } else { full_impl_span . shrink_to_hi () } ; let padding = tcx . sess . source_map () . indentation_before (sugg_sp) . unwrap_or_else (String :: new) ; let (mut missing_trait_item , mut missing_trait_item_none , mut missing_trait_item_label) = (Vec :: new () , Vec :: new () , Vec :: new ()) ; for & trait_item in missing_items { let snippet = with_types_for_signature ! (suggestion_signature (tcx , trait_item , tcx . impl_trait_ref (impl_def_id) . unwrap () . instantiate_identity () ,)) ; let code = format ! ("{padding}{snippet}\n{padding}") ; if let Some (span) = tcx . hir_span_if_local (trait_item . def_id) { missing_trait_item_label . push (errors :: MissingTraitItemLabel { span , item : trait_item . name () }) ; missing_trait_item . push (errors :: MissingTraitItemSuggestion { span : sugg_sp , code , snippet , }) ; } else { missing_trait_item_none . push (errors :: MissingTraitItemSuggestionNone { span : sugg_sp , code , snippet , }) } } tcx . dcx () . emit_err (errors :: MissingTraitItem { span : tcx . span_of_impl (impl_def_id . to_def_id ()) . unwrap () , missing_items_msg , missing_trait_item_label , missing_trait_item , missing_trait_item_none , }) ; }
    };
}

missing_items_err!();