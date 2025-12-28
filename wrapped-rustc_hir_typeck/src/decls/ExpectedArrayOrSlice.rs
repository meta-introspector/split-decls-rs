macro_rules! deps {
    () => {
        SlicingSuggestion!();
        AsDerefSuggestion!();
    };
}

macro_rules! ExpectedArrayOrSlice {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_typeck_expected_array_or_slice , code = E0529)] pub (crate) struct ExpectedArrayOrSlice < 'tcx > { # [primary_span] # [label (hir_typeck_expected_array_or_slice_label)] pub (crate) span : Span , pub (crate) ty : Ty < 'tcx > , pub (crate) slice_pat_semantics : bool , # [subdiagnostic] pub (crate) as_deref : Option < AsDerefSuggestion > , # [subdiagnostic] pub (crate) slicing : Option < SlicingSuggestion > , }
    };
}

ExpectedArrayOrSlice!()