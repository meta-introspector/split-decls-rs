macro_rules! deps {
    () => {
        UniversalRegionRelations!();
        MirTypeckRegionConstraints!();
        BorrowckInferCtxt!();
    };
}

macro_rules! clone_and_resolve_opaque_types {
    () => {
        deps!();
        # [doc = " We eagerly map all regions to NLL vars here, as we need to make sure we've"] # [doc = " introduced nll vars for all used placeholders."] # [doc = ""] # [doc = " We need to resolve inference vars as even though we're in MIR typeck, we may still"] # [doc = " encounter inference variables, e.g. when checking user types."] pub (crate) fn clone_and_resolve_opaque_types < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , universal_region_relations : & Frozen < UniversalRegionRelations < 'tcx > > , constraints : & mut MirTypeckRegionConstraints < 'tcx > ,) -> (OpaqueTypeStorageEntries , Vec < (OpaqueTypeKey < 'tcx > , OpaqueHiddenType < 'tcx >) >) { let opaque_types = infcx . clone_opaque_types () ; let opaque_types_storage_num_entries = infcx . inner . borrow_mut () . opaque_types () . num_entries () ; let opaque_types = opaque_types . into_iter () . map (| entry | { fold_regions (infcx . tcx , infcx . resolve_vars_if_possible (entry) , | r , _ | { let vid = if let ty :: RePlaceholder (placeholder) = r . kind () { constraints . placeholder_region (infcx , placeholder) . as_var () } else { universal_region_relations . universal_regions . to_region_vid (r) } ; Region :: new_var (infcx . tcx , vid) }) }) . collect :: < Vec < _ > > () ; (opaque_types_storage_num_entries , opaque_types) }
    };
}

clone_and_resolve_opaque_types!()